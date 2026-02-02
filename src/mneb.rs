use anyhow::{Result, ensure};
use byteorder::{BigEndian, ReadBytesExt};
use serde::Serialize;
use std::io::{Cursor, Seek};

#[derive(Default, Debug, Serialize)]
pub struct ControlPoint {
    pub x: i16,
    pub y: i16,
    pub z: i16,
    pub w: i16,
}

impl ControlPoint {
    fn from_bytes(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let x = cursor.read_i16::<BigEndian>()?;
        let y = cursor.read_i16::<BigEndian>()?;
        let z = cursor.read_i16::<BigEndian>()?;
        let w = cursor.read_i16::<BigEndian>()?;

        Ok(Self { x, y, z, w })
    }
}

#[derive(Default, Debug, Serialize)]
pub struct KeyFrame {
    pub frame: u16,
    pub is_active: bool,
    pub x: i16, // these are positions, not offsets
    pub y: i16,
}

impl KeyFrame {
    fn from_bytes(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let frame = cursor.read_u16::<BigEndian>()?;
        let is_active = cursor.read_u8()? != 0;
        let _ = cursor.seek_relative(1);
        let x = cursor.read_i16::<BigEndian>()?;
        let y = cursor.read_i16::<BigEndian>()?;

        Ok(Self {
            frame,
            is_active,
            x,
            y,
        })
    }
}

#[derive(Default, Debug, Serialize)]
pub struct KeyFrameSet {
    pub node_index: u16,
    pub key_frames: Vec<KeyFrame>,
}

impl KeyFrameSet {
    fn from_bytes(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let node_index = cursor.read_u16::<BigEndian>()?;
        let num_key_frames = cursor.read_u16::<BigEndian>()?;

        let mut key_frames: Vec<KeyFrame> = Vec::with_capacity(num_key_frames as usize);

        for _ in 0..num_key_frames {
            key_frames.push(KeyFrame::from_bytes(cursor)?);
        }

        Ok(Self {
            node_index,
            key_frames,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct Curve {
    pub name: String,
    pub control_points: Vec<ControlPoint>,
    pub knots: Vec<f32>,
    pub key_frame_sets: Vec<KeyFrameSet>,

    /* unknown fields */
    #[serde(serialize_with = "<[_]>::serialize")]
    pub unk_28: [u8; 0x64],
    pub unk_8c: f32,
    pub unk_90: u32,
    pub unk_94: u32,
    pub unk_98: u32,
    pub unk_a8: [f32; 4],
    pub unk_b8: Vec<u8>,
}

impl Default for Curve {
    // necessary because we can't derive Default for arrays with a length greather than 32
    // ...which means we can't use ..Default::default either due to recursion
    fn default() -> Curve {
        Curve {
            name: String::new(),
            control_points: Vec::new(),
            knots: Vec::new(),
            key_frame_sets: Vec::new(),
            unk_28: [0u8; 0x64],
            unk_8c: 0.0f32,
            unk_90: 0,
            unk_94: 0,
            unk_98: 0,
            unk_a8: [0.0f32; 4],
            unk_b8: Vec::new(),
        }
    }
}

#[derive(Default, Debug, Serialize)]
pub struct DemoOption {
    pub name: String,
    pub value: String,
}

#[derive(Default, Debug, Serialize)]
pub struct DemoOptionSet {
    pub name: String,
    pub demo_options: Vec<DemoOption>,

    /* unknown fields */
    pub unk_20: [u8; 0x20],
}

#[derive(Default, Debug, Serialize)]
pub struct MNEBFile {
    pub curves: Vec<Curve>,
    pub demo_option_sets: Vec<DemoOptionSet>,
    pub frame_count: u16,

    /* unknown fields */
    pub _unk_8: u32,
    pub _unk_10: u32,
    pub _unk_16: bool,
}

impl MNEBFile {
    pub fn has_curves(&self) -> bool {
        !self.curves.is_empty()
    }

    pub fn from_bytes(raw: &[u8]) -> Result<Self, anyhow::Error> {
        let mut c = Cursor::new(raw);

        let header_magic = c.read_u32::<BigEndian>()?.to_be_bytes();
        ensure!(&header_magic == b"MNCH", "Invalid file header.");

        let data_offset = c.read_u32::<BigEndian>()?;
        let unk_8 = c.read_u32::<BigEndian>()?;
        let num_curves = c.read_u32::<BigEndian>()?;
        let unk_10 = c.read_u32::<BigEndian>()?;
        let frame_count = c.read_u16::<BigEndian>()?;
        let unk_16 = c.read_u8()? != 0;
        let _ = c.seek_relative(1);

        let mut demo_option_sets: Vec<DemoOptionSet> = Vec::new();
        let mut curves: Vec<Curve> = Vec::with_capacity(num_curves as usize);
        // check block type

        if num_curves == 0 {
            // demo data
            let magic = c.read_u32::<BigEndian>()?.to_be_bytes();
            ensure!(
                &magic == b"MNDD",
                format!("Invalid demo data header at offset {:X}", c.position() - 4)
            );
            let _ = c.read_u32::<BigEndian>()?; // block size, but not relevant
            let num_demo_option_sets = c.read_u32::<BigEndian>()?;

            for _ in 0..num_demo_option_sets {
                let cur_pos = c.position();

                // read demo option set
                let offset = c.read_u32::<BigEndian>()?;
                c.set_position(offset as u64);

                let name = {
                    let pos = c.position() as usize;
                    let _ = c.seek_relative(0x20);

                    let raw = c.get_ref();
                    let mut name_vec = raw[pos..pos + 0x20].to_vec();
                    name_vec.retain(|b| *b != 0);
                    String::from_utf8(name_vec)?
                };

                let unk_20 = {
                    let mut temp = [0u8; 0x20];
                    let pos = c.position() as usize;
                    let _ = c.seek_relative(0x20);

                    let raw = c.get_ref();
                    let v = raw[pos..pos + 0x20].to_vec();
                    temp.copy_from_slice(&v);

                    temp
                };

                let option_count = c.read_u32::<BigEndian>()?;

                let mut demo_options: Vec<DemoOption> = Vec::with_capacity(option_count as usize);
                for _ in 0..option_count {
                    let cur_pos = c.position();

                    let offset = c.read_u32::<BigEndian>()?;
                    c.set_position(offset as u64);

                    // read demo options
                    let name = {
                        let pos = c.position() as usize;
                        let _ = c.seek_relative(0x10);

                        let raw = c.get_ref();
                        let mut name_vec = raw[pos..pos + 0x10].to_vec();
                        name_vec.retain(|b| *b != 0);
                        String::from_utf8(name_vec)?
                    };

                    let num_values = c.read_u32::<BigEndian>()?;
                    let mut raw_string_bytes: Vec<u8> = Vec::with_capacity(num_values as usize);
                    let pos = c.position() as usize;
                    let raw = c.get_ref();
                    raw_string_bytes.extend_from_slice(&raw[pos..pos + num_values as usize]);
                    let value = String::from_utf8(raw_string_bytes)?;
                    demo_options.push(DemoOption { name, value });
                    c.set_position(cur_pos + 4);
                }

                demo_option_sets.push(DemoOptionSet {
                    name,
                    demo_options,
                    unk_20,
                });
                c.set_position(cur_pos + 4);
            }
        } else {
            c.set_position(data_offset as u64);
            for _ in 0..num_curves {
                let start = c.position() as usize;
                let magic = c.read_u32::<BigEndian>()?.to_be_bytes();
                ensure!(
                    &magic == b"MNCN",
                    format!("Invalid curve header at offset {:X}", c.position() - 4)
                );

                let block_size = c.read_u32::<BigEndian>()? as usize;

                let offset_to_next = start + block_size;

                let name = {
                    let pos = c.position() as usize;
                    let _ = c.seek_relative(0x20);

                    let raw = c.get_ref();
                    let mut name_vec = raw[pos..pos + 0x20].to_vec();
                    name_vec.retain(|b| *b != 0);

                    String::from_utf8(name_vec)?
                };

                let unk_28 = {
                    let mut temp: [u8; 0x64] = [0u8; 0x64];

                    let pos = c.position() as usize;
                    let _ = c.seek_relative(0x64);

                    let raw = c.get_ref();

                    temp.copy_from_slice(&raw[pos..pos + 0x64]);

                    temp
                };

                let unk_8c = c.read_f32::<BigEndian>()?;
                let unk_90 = c.read_u32::<BigEndian>()?;
                let unk_94 = c.read_u32::<BigEndian>()?;
                let unk_98 = c.read_u32::<BigEndian>()?;
                let control_point_table_offset = c.read_u32::<BigEndian>()?;
                let knot_table_offset = c.read_u32::<BigEndian>()?;
                let key_frame_info_offset = c.read_u32::<BigEndian>()?;
                let unk_a8 = {
                    let pos = c.position() as usize;
                    let _ = c.seek_relative(0x10);

                    let raw = c.get_ref();
                    let float_slice = bytemuck::cast_slice(&raw[pos..pos + 0x10]);
                    let mut temp = [0f32; 4];
                    temp.copy_from_slice(float_slice);

                    temp
                };

                // read any extra data
                let mut unk_b8 =
                    Vec::with_capacity((control_point_table_offset as u64 - c.position()) as usize);
                {
                    let pos = c.position() as usize;
                    let raw = c.get_ref();
                    unk_b8.extend_from_slice(&raw[pos..pos + unk_b8.capacity()])
                }

                // read control points
                c.set_position(control_point_table_offset as u64);
                let num_control_points = c.read_u32::<BigEndian>()?;
                let mut control_points: Vec<ControlPoint> =
                    Vec::with_capacity(num_control_points as usize);

                for _ in 0..num_control_points {
                    control_points.push(ControlPoint::from_bytes(&mut c)?);
                }

                // read knots
                c.set_position(knot_table_offset as u64);
                let num_knots = c.read_u32::<BigEndian>()?;
                let mut knots: Vec<f32> = Vec::with_capacity(num_knots as usize);

                for _ in 0..num_knots {
                    knots.push(c.read_f32::<BigEndian>()?);
                }

                // read key frame info
                c.set_position(key_frame_info_offset as u64);
                let key_frame_table_offset = c.read_u32::<BigEndian>()?;
                c.set_position(key_frame_table_offset as u64);

                let num_key_frame_sets = c.read_u32::<BigEndian>()?;

                let mut key_frame_sets: Vec<KeyFrameSet> =
                    Vec::with_capacity(num_key_frame_sets as usize);

                for _ in 0..num_key_frame_sets {
                    let cur_offset = c.position();

                    let offset = c.read_u32::<BigEndian>()?;
                    c.set_position(offset as u64);

                    key_frame_sets.push(KeyFrameSet::from_bytes(&mut c)?);

                    c.set_position(cur_offset + 4);
                }

                let curve = Curve {
                    name,
                    control_points,
                    knots,
                    key_frame_sets,
                    unk_28,
                    unk_8c,
                    unk_90,
                    unk_94,
                    unk_98,
                    unk_a8,
                    unk_b8,
                };

                curves.push(curve);

                // go to next curve block
                c.set_position(offset_to_next as u64);
            }
        }

        Ok(Self {
            curves,
            demo_option_sets,
            frame_count,
            _unk_8: unk_8,
            _unk_10: unk_10,
            _unk_16: unk_16,
        })
    }
}

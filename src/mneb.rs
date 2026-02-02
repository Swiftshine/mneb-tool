use anyhow::{Result, ensure};
use byteorder::{BigEndian, ByteOrder, ReadBytesExt};
use derivative::{self, Derivative};
use std::io::{Cursor, Seek};

#[derive(Default, Debug)]
pub struct ControlPoint {
    pub x: i16,
    pub y: i16,
    pub z: i16,
    pub w: i16,
}

#[derive(Default, Debug)]
pub struct KeyFrame {
    pub frame: u16,
    pub is_active: bool,
    pub x: i16,
    pub y: i16,
}

#[derive(Default, Debug)]
pub struct KeyFrameSet {
    pub node_index: u16,
    pub key_frames: Vec<KeyFrame>,
}

#[derive(Derivative)]
#[derivative(Default, Debug)]
pub struct Curve {
    pub name: String,
    pub control_points: Vec<ControlPoint>,
    pub knots: Vec<f32>,
    pub key_frame_sets: Vec<KeyFrameSet>,

    /* unknown fields */
    #[derivative(Default(value = "[0u8; 0x64]"))]
    pub unk_28: [u8; 0x64],
    pub unk_8c: f32,
    pub unk_90: u32,
    pub unk_94: u32,
    pub unk_98: u32,
    pub unk_a8: [f32; 4], // pub unk_b8:
}

#[derive(Default, Debug)]
pub struct DemoOption {
    pub name: String,
    pub values: Vec<u8>,
}

#[derive(Default, Debug)]
pub struct DemoOptionSet {
    pub name: String,
    pub demo_options: Vec<DemoOption>,

    /* unknown fields */
    pub unk_20: [u8; 0x20],
}

#[derive(Default, Debug)]
pub struct MNEBFile {
    pub curves: Vec<Curve>,
    pub demo_option_sets: Vec<DemoOptionSet>,
    pub is_animated: bool,
    pub frame_count: u16,

    /* unknown fields */
    pub unk_8: u32,
    pub unk_10: u32,
}

impl MNEBFile {
    pub fn has_curves(&self) -> bool {
        !self.curves.is_empty()
    }

    pub fn from_bytes(raw: &[u8]) -> Result<Self, anyhow::Error> {
        ensure!(&raw[..4] == b"MNCH", "Invalid file header.");

        let mut c = Cursor::new(raw);
        c.set_position(4);

        let curve_block_offset = c.read_u32::<BigEndian>()? as usize;
        let header_unk_8 = c.read_u32::<BigEndian>()?;
        let curve_block_count = c.read_u32::<BigEndian>()?;
        let header_unk_10 = c.read_u32::<BigEndian>()?;
        let frame_count = c.read_u16::<BigEndian>()?;
        let header_unk_16 = c.read_u8()? != 0;
        let _ = c.seek_relative(1);

        // check block type

        if curve_block_count == 0 {
            // demo data
            todo!()
        } else {
            todo!()
        }

        Ok(Self::default())
    }
}

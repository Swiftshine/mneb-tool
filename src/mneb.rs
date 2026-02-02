// #[derive(Default, Debug)]
// pub struct ControlPoint {
//     pub x: i16,
//     pub y: i16,
//     pub z: i16,
//     pub w: i16
// }

// #[derive(Default, Debug)]
// pub struct KeyFrame {
//     pub frame: u16,
//     pub is_active: bool,
//     pub x: i16,
//     pub y: i16
// }

// #[derive(Default, Debug)]
// pub struct KeyFrameSet {
//     pub node_index: u16,
//     pub key_frames: Vec<KeyFrame>
// }

// #[derive(Debug)]
// pub struct Curve {
//     pub name: String,
//     pub control_points: Vec<ControlPoint>,
//     pub knots: Vec<f32>,
//     pub key_frame_sets: Vec<KeyFrameSet>,

//     /* unknown fields */

//     pub unk_28: [u8; 0x64],
//     pub unk_8c: f32,
//     pub unk_90: u32,
//     pub unk_94: u32,
//     pub unk_98: u32,
//     pub unk_a8: [f32; 4]
//     // pub unk_b8:
// }

// impl Default for Curve {
//     fn default() -> Self {
//         Self {
//             unk_28: [Default::default(); 0x64],
//             ..Default::default()
//         }
//     }
// }

// #[derive(Default, Debug)]
// pub struct DemoOption {
//     pub name: String,
//     pub values: Vec<u8>
// }

// #[derive(Default, Debug)]
// pub struct DemoOptionSet {
//     pub name: String,
//     pub demo_options: Vec<DemoOption>,

//     /* unknown fields */
//     pub unk_20: [u8; 0x20]
// }


// #[derive(Default, Debug)]
// pub struct MNEBFile {
//     pub curves: Vec<Curve>,
//     pub demo_option_sets: Vec<DemoOptionSet>,
//     pub is_animated: bool,
//     pub frame_count: u16,

//     /* unknown fields */

//     pub unk_8: u32,
//     pub unk_10: u32,
// }

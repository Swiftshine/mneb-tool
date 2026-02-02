use crate::mneb::*;
use macroquad::prelude::*;

pub fn do_graph(mneb_file: MNEBFile, framerate: f32) {
    let conf = Conf {
        window_title: "MNEB Renderer".to_string(),
        ..Default::default()
    };

    let mut current_frame = 0.0f32;

    macroquad::Window::from_config(conf, async move {
        loop {
            clear_background(WHITE);

            // update frame
            current_frame =
                (current_frame + framerate * get_frame_time()) % mneb_file.frame_count as f32;

            // iterate through curves

            for curve in mneb_file.curves.iter() {
                todo!()
            }

            next_frame().await
        }
    });
}

fn interpolate(keyframes: &[KeyFrame], current_frame: f32) -> Option<(f32, f32)> {
    if keyframes.is_empty() {
        return None;
    }

    if keyframes.len() == 1 {
        return Some((keyframes[0].x as f32, keyframes[0].y as f32));
    }

    for i in 0..keyframes.len() - 1 {
        let start = &keyframes[i];
        let end = &keyframes[i + 1];

        if current_frame >= start.frame as f32 && current_frame <= end.frame as f32 {
            let t = (current_frame - start.frame as f32) / (end.frame as f32 - start.frame as f32);
            let x = start.x as f32 + t * (end.x as f32 - start.x as f32);
            let y = start.y as f32 + t * (end.y as f32 - start.y as f32);
            return Some((x, y));
        }
    }

    keyframes.last().map(|k| (k.x as f32, k.y as f32))
}

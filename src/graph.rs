use crate::mneb::*;
use macroquad::prelude::*;

pub fn do_graph(mneb_file: MNEBFile, framerate: f32) {
    let conf = Conf {
        window_title: "MNEB Renderer".to_string(),
        ..Default::default()
    };

    macroquad::Window::from_config(conf, async move {
        let mut current_frame = 0.0f32;
        let mut camera_pos = Vec2::splat(0.0);
        let mut zoom = 0.0001f32;

        loop {
            /* config updates */
            clear_background(WHITE);

            // update frame
            current_frame =
                (current_frame + framerate * get_frame_time()) % mneb_file.frame_count as f32;

            // handle zoom
            let wheel = mouse_wheel();
            if wheel.1 != 0.0 {
                let zoom_factor = 1.1f32;
                if wheel.1 > 0.0 {
                    zoom *= zoom_factor;
                } else {
                    zoom /= zoom_factor;
                }
            }

            let camera = Camera2D {
                target: camera_pos,
                zoom: vec2(zoom * (screen_height() / screen_width()), -zoom),
                ..Default::default()
            };
            set_camera(&camera);

            /* drawing */

            // iterate through curves

            for curve in mneb_file.curves.iter() {
                let mut points = Vec::new();
                for i in 0..curve.control_points.len() - 1 {
                    let control_point = &curve.control_points[i];
                    points.push((control_point.x as f32, control_point.y as f32));
                }

                // draw points
                for (i, point) in points.iter().enumerate() {
                    draw_circle(point.0, point.1, 7.0 / (zoom * 500.0), RED);
                }
                // draw lines between points

                for i in 0..points.len() - 1 {
                    draw_line(
                        points[i].0,
                        points[i].1,
                        points[i + 1].0,
                        points[i + 1].1,
                        5.0 / (zoom * 500.0),
                        BLUE,
                    );
                }

                // draw the last line
                let first = &points[0];
                let last = &points[points.len() - 1];

                // draw_line(last.0, last.1, first.0, first.1, 2.0 / (zoom * 500.0), BLUE);
            }

            // render text
            set_default_camera();

            draw_text(
                &format!(
                    "Frame: {:.0} / {}",
                    current_frame, mneb_file.frame_count as f32
                ),
                20.0,
                20.0,
                25.0,
                BLACK,
            );

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

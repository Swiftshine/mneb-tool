use crate::mneb::*;
use macroquad::prelude::*;

const FONT_SIZE: f32 = 20.0f32;

pub fn animate_files(mneb_files: Vec<(String, MNEBFile)>, framerate: f32) {
    let conf = Conf {
        window_title: "MNEB Renderer".to_string(),
        ..Default::default()
    };

    macroquad::Window::from_config(conf, async move {
        let mut current_anim_index = 0;
        let mut current_frame = 0.0f32;
        let mut camera_pos = Vec2::splat(0.0);
        let mut zoom = 0.001f32;
        let mut last_mouse_pos: Vec2 = mouse_position().into();
        let mut paused = false;

        loop {
            let (filename, mneb_file) = &mneb_files[current_anim_index];

            /* config updates */

            clear_background(WHITE);

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

            // mouse input
            let mouse_pos: Vec2 = mouse_position().into();
            let mouse_delta = mouse_pos - last_mouse_pos;
            last_mouse_pos = mouse_pos;

            if is_mouse_button_down(MouseButton::Left) {
                camera_pos -= mouse_delta / (zoom * screen_height());
            }

            // pause
            if is_key_pressed(KeyCode::Space) {
                paused = !paused;
            }

            // update frame
            if !paused {
                current_frame += framerate * get_frame_time();

                // if done, move to the next one
                if current_frame >= mneb_file.frame_count as f32 {
                    current_frame = 0.0;
                    current_anim_index = (current_anim_index + 1) % mneb_files.len();
                }
            }

            /* drawing */

            let camera = Camera2D {
                target: camera_pos,
                zoom: vec2(zoom * (screen_height() / screen_width()), -zoom),
                ..Default::default()
            };
            set_camera(&camera);

            animate_curves(&mneb_file.curves, zoom, current_frame);

            // render text
            set_default_camera();

            draw_text(
                &format!(
                    "Playing {}/{}: {}",
                    current_anim_index,
                    mneb_files.len(),
                    &filename
                ),
                20.0,
                15.0,
                FONT_SIZE,
                BLACK,
            );
            draw_text(
                "Space: Pause | Left-click: Pan | Scroll: Zoom",
                20.0,
                30.0,
                FONT_SIZE,
                BLACK,
            );
            draw_text(
                &format!(
                    "Frame: {:.0} / {}",
                    current_frame, mneb_file.frame_count as f32
                ),
                20.0,
                45.0,
                FONT_SIZE,
                BLACK,
            );
            draw_text(&format!("Zoom: {:.4}", zoom), 20.0, 60.0, FONT_SIZE, BLACK);

            next_frame().await
        }
    });
}

pub fn animate_file(mneb_file: MNEBFile, framerate: f32, filename: String) {
    let conf = Conf {
        window_title: "MNEB Renderer".to_string(),
        ..Default::default()
    };

    macroquad::Window::from_config(conf, async move {
        let mut current_frame = 0.0f32;
        let mut camera_pos = Vec2::splat(0.0);
        let mut zoom = 0.001f32;
        let mut last_mouse_pos: Vec2 = mouse_position().into();
        let mut paused = false;

        loop {
            /* config updates */

            clear_background(WHITE);

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

            // mouse input
            let mouse_pos: Vec2 = mouse_position().into();
            let mouse_delta = mouse_pos - last_mouse_pos;
            last_mouse_pos = mouse_pos;

            if is_mouse_button_down(MouseButton::Left) {
                camera_pos -= mouse_delta / (zoom * screen_height());
            }

            // pause
            if is_key_pressed(KeyCode::Space) {
                paused = !paused;
            }

            // update frame
            if !paused {
                current_frame =
                    (current_frame + framerate * get_frame_time()) % mneb_file.frame_count as f32;
            }

            /* drawing */

            let camera = Camera2D {
                target: camera_pos,
                zoom: vec2(zoom * (screen_height() / screen_width()), -zoom),
                ..Default::default()
            };
            set_camera(&camera);

            animate_curves(&mneb_file.curves, zoom, current_frame);

            // render text
            set_default_camera();

            draw_text(
                &format!("Playing: {}", &filename),
                20.0,
                15.0,
                FONT_SIZE,
                BLACK,
            );
            draw_text(
                "Space: Pause | Left-click: Pan | Scroll: Zoom",
                20.0,
                30.0,
                FONT_SIZE,
                BLACK,
            );
            draw_text(
                &format!(
                    "Frame: {:.0} / {}",
                    current_frame, mneb_file.frame_count as f32
                ),
                20.0,
                45.0,
                FONT_SIZE,
                BLACK,
            );
            draw_text(&format!("Zoom: {:.4}", zoom), 20.0, 60.0, FONT_SIZE, BLACK);

            next_frame().await
        }
    });
}

fn animate_curves(curves: &[Curve], zoom: f32, current_frame: f32) {
    for curve in curves.iter() {
        let mut current_positions: Vec<Vec2> = curve
            .control_points
            .iter()
            .map(|point| vec2(point.x as f32, point.y as f32))
            .collect();

        for key_set in &curve.key_frame_sets {
            let idx = key_set.node_index as usize;
            if idx < current_positions.len()
                && let Some((nx, ny)) = interpolate(&key_set.key_frames, current_frame) {
                    current_positions[idx] = vec2(nx, ny);
                }
        }

        // draw lines
        if current_positions.len() > 1 {
            for i in 0..current_positions.len() - 1 {
                draw_line(
                    current_positions[i].x,
                    current_positions[i].y,
                    current_positions[i + 1].x,
                    current_positions[i + 1].y,
                    2.0 / (zoom * 500.0), // line thickness
                    BLUE,
                );
            }
        }

        // // draw points
        // for pos in &current_positions {
        //     draw_circle(pos.x, pos.y, 3.0, RED);
        // }
    }
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

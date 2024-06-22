use raycasting::RayCasting;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;

mod raycasting;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("raycasting", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())?;

    let mut raycasting = RayCasting::default();

    'mainloop: loop {
        let mut move_factor = 0.0;
        let mut rotation_factor = 0.0;
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                | Event::Quit { .. } => break 'mainloop,
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => move_factor += 0.05,
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => move_factor -= 0.05,
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => rotation_factor += 0.001,
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => rotation_factor -= 0.001,
                _ => {}
            }
        }

        raycasting.transform_cam(move_factor, rotation_factor);
        let lines = raycasting.lines(800, 600);

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        for line in lines {
            let mut color = match line.2 {
                1 => Color::RED,
                2 => Color::GREEN,
                3 => Color::BLUE,
                4 => Color::WHITE,
                _ => Color::YELLOW,
            };
            if line.3 {
                color.r = color.r / 2;
                color.g = color.g / 2;
                color.b = color.b / 2;
            }
            canvas.set_draw_color(color);
            canvas
                .draw_line(
                    Point::new(line.0.x as i32, line.0.y as i32),
                    Point::new(line.1.x as i32, line.1.y as i32),
                )
                .unwrap();
        }
        canvas.present();
    }

    Ok(())
}

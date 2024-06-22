use std::time::Duration;

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
        .window("Ray Tracing", 800, 600)
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
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                | Event::Quit { .. } => break 'mainloop,
                _ => {}
            }
        }

        let lines = raycasting.lines(Duration::ZERO, 800, 600);

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        for line in lines {
            let color = match line.2 {
                1 => Color::RED,
                2 => Color::GREEN,
                3 => Color::BLUE,
                4 => Color::WHITE,
                _ => Color::YELLOW,
            };
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

use raycasting::RayCasting;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};

mod raycasting;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("raycasting", WINDOW_WIDTH, WINDOW_HEIGHT)
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
                Event::Quit { .. } => break 'mainloop,
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
                } => rotation_factor += 0.03,
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => rotation_factor -= 0.03,
                _ => {}
            }
        }

        raycasting.transform_cam(move_factor, rotation_factor);
        let lines = raycasting.lines(WINDOW_WIDTH as u16, WINDOW_HEIGHT as u16);

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // render the world
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

        // render the map
        for (y, row) in raycasting.world.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let color = match cell {
                    1 => Color::RED,
                    2 => Color::GREEN,
                    3 => Color::BLUE,
                    4 => Color::WHITE,
                    _ => Color::YELLOW,
                };
                canvas.set_draw_color(color);
                let rect = Rect::new(x as i32 * 5, y as i32 * 5, 5, 5);
                canvas.fill_rect(rect).unwrap();
            }
        }
        canvas.set_draw_color(Color::BLACK);
        canvas
            .fill_rect(Rect::new(
                raycasting.pos.x as i32 * 5 - 1,
                raycasting.pos.y as i32 * 5 - 1,
                3,
                3,
            ))
            .unwrap();
        canvas
            .draw_line(
                Point::new(raycasting.pos.x as i32 * 5, raycasting.pos.y as i32 * 5),
                Point::new(
                    ((raycasting.pos + raycasting.dir).x * 5.0) as i32,
                    ((raycasting.pos + raycasting.dir).y * 5.0) as i32,
                ),
            )
            .unwrap();

        println!("{}", raycasting.dir);

        canvas.present();
    }

    Ok(())
}

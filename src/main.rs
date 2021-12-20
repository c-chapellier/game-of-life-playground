
extern crate sdl2;
extern crate rand;
extern crate time;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;

use rand::Rng;

use std::time::Duration;
use std::time::Instant;

const WIN_HEIGHT: usize = 800;
const WIN_WIDTH: usize = 800;

const CELL_HEIGHT: usize = 20;
const CELL_WIDTH: usize = 20;
const BORDER_SIZE: usize = 1;

const HEIGHT: usize = WIN_HEIGHT / (CELL_HEIGHT + 1);
const WIDTH: usize = WIN_WIDTH / (CELL_WIDTH + 1);

struct Colors {
    alive_cell_color: sdl2::pixels::Color,
    dead_cell_color: sdl2::pixels::Color,
    border_color: sdl2::pixels::Color
}

fn main() -> std::io::Result<()> {

    let colors = Colors {
        alive_cell_color: Color::RGB(136, 37, 98),
        dead_cell_color: Color::RGB(30, 30, 30),
        border_color: Color::RGB(0, 0, 0)
    };

    assert_eq!(WIN_WIDTH % (CELL_WIDTH + BORDER_SIZE), BORDER_SIZE);
    
    let mut cells = [[0u8; WIN_WIDTH / (CELL_WIDTH + BORDER_SIZE)]; WIN_HEIGHT / (CELL_HEIGHT + BORDER_SIZE)];

    let mut rng = rand::thread_rng();

    let (mut canvas, mut event_pump) = init_sdl();

    let mut sum: u128 = 0;
    let mut n = 0;

    'running: loop {

        let now = Instant::now();

        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                cells[i][j] = rng.gen_range(0..2);
            }
        }

        draw_cells(&mut canvas, &cells, &colors);

        let elapsed = now.elapsed();
        sum += elapsed.as_micros();
        n += 1;

        println!("Elapsed: {:.2?}", elapsed);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        let now2 = Instant::now();

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 4));

        let elapsed2 = now2.elapsed();

        println!("sleep: {:.2?}", elapsed2);

        // if n > 20 {
        //     break ;
        // }

    }

    println!("Mean: {:.2?}µs", sum / n);

    Ok(())
}

fn init_sdl() -> (sdl2::render::Canvas<sdl2::video::Window>, sdl2::EventPump) {

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", WIN_WIDTH as u32, WIN_HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    canvas.present();

    (canvas, sdl_context.event_pump().unwrap())
}

fn draw_cells(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, cells: &[[u8; 38]; 38], colors: &Colors) {

    canvas.set_draw_color(colors.border_color);
    canvas.clear();

    canvas.set_draw_color(colors.alive_cell_color);

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            if cells[i][j] == 1 {
                canvas.fill_rect(
                    Rect::new(
                        (BORDER_SIZE + ((CELL_WIDTH + BORDER_SIZE) * j)) as i32,
                        (BORDER_SIZE + ((CELL_HEIGHT + BORDER_SIZE) * i)) as i32,
                        CELL_WIDTH as u32,
                        CELL_HEIGHT as u32)
                ).unwrap();
            }
        }
    }

    canvas.set_draw_color(colors.dead_cell_color);

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            if cells[i][j] == 0 {
                canvas.fill_rect(
                    Rect::new(
                        (BORDER_SIZE + ((CELL_WIDTH + BORDER_SIZE) * j)) as i32,
                        (BORDER_SIZE + ((CELL_HEIGHT + BORDER_SIZE) * i)) as i32,
                        CELL_WIDTH as u32,
                        CELL_HEIGHT as u32)
                ).unwrap();
            }
        }
    }
}

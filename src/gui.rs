
pub struct GUI {
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    alive_cell_color: sdl2::pixels::Color,
    dead_cell_color: sdl2::pixels::Color,
    border_color: sdl2::pixels::Color
}

pub fn create_gui () -> (GUI, sdl2::EventPump) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    
    let window = video_subsystem.window("rust-sdl2 demo", crate::WIN_WIDTH as u32, crate::WIN_HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();
    
    let mut canvas = window.into_canvas().build().unwrap();
    
    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    
    canvas.present();

    (GUI {
        canvas: canvas,
        alive_cell_color: sdl2::pixels::Color::RGB(136, 37, 98),
        dead_cell_color: sdl2::pixels::Color::RGB(30, 30, 30),
        border_color: sdl2::pixels::Color::RGB(0, 0, 0)
    }, sdl_context.event_pump().unwrap())
}

impl GUI {

    pub fn refresh(&mut self) {
        self.canvas.present();
    }

    pub fn draw_cell(&mut self, i: usize, j: usize, state: u8) {
        self.canvas.set_draw_color(if state == crate::ALIVE { self.alive_cell_color } else { self.dead_cell_color });
        self.canvas.fill_rect(
            sdl2::rect::Rect::new(
                (crate::BORDER_SIZE + ((crate::CELL_WIDTH + crate::BORDER_SIZE) * j)) as i32,
                (crate::BORDER_SIZE + ((crate::CELL_HEIGHT + crate::BORDER_SIZE) * i)) as i32,
                crate::CELL_WIDTH as u32,
                crate::CELL_HEIGHT as u32)
        ).unwrap();
    }
    
    pub fn draw_cells(&mut self, cells: &[[u8; crate::WIDTH]; crate::HEIGHT]) {
        
        self.canvas.set_draw_color(self.border_color);
        self.canvas.clear();
    
        for i in 0..crate::HEIGHT {
            for j in 0..crate::WIDTH {
                self.draw_cell(i, j, cells[i][j]);
            }
        }
    }    
}

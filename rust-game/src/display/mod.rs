use raylib::{ffi::DrawCube, prelude::*};

use crate::level::Level;

pub struct Display
{
    pub rl: RaylibHandle,
    thread: RaylibThread,
    cam:    Camera3D,
}

impl Display
{
    pub fn new() -> Self
    {
        let (mut rl, thread) = raylib::init().build();
        let mut cam = Camera3D::perspective(
            Vector3 {
                x: -50.0,
                y: 100.0,
                z: -50.0,
            },
            Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Vector3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            45.0,
        );
        return Self {
            rl,
            thread,
            cam,
        };
    }

    fn draw_3d(d: &mut RaylibMode3D<RaylibDrawHandle>, lvl: &Level)
    {
        lvl.terrain.draw(d);
    }

    pub fn draw_loop(&mut self, lvl: &Level)
    {
        let mut d = self.rl.begin_drawing(&self.thread);
        d.clear_background(Color::WHITE);

        {
            let mut d3d = d.begin_mode3D(self.cam);
            Self::draw_3d(&mut d3d, lvl);
        }

        // 2d drawing
        d.draw_text("Hello from Joe", 20, 20, 20, Color::BLUE);
        d.draw_text(&d.get_fps().to_string(), 20, 40, 20, Color::BLACK );

    }
}

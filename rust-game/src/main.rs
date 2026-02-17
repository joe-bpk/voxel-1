mod display;
mod level;

fn main()
{
    let mut display = display::Display::new();

    let mut lvl = level::Level::new();
    lvl.terrain.perlinify();
    display.rl.set_target_fps(100);
    lvl.update();

    display.load_lvl(&lvl);

    while !display.rl.window_should_close() {
        {
            lvl.update();
            display.draw_loop();
        }
    }
}

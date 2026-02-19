mod chunk_loader;
mod display;
mod level;

use crate::chunk_loader::ChunkWorkerPool;
use crate::level::terrain::DynTerr;
use std::sync::{Arc, Mutex};

fn main()
{
    // initialize display and frame rate
    let mut display = display::Display::new();
    display.rl.set_target_fps(1000);

    // setup terrain data and thread pool
    let terr = Arc::new(Mutex::new(DynTerr::new()));
    let mut pool = ChunkWorkerPool::new(Arc::clone(&terr));

    if !display.rl.window_should_close() {
        display.draw_loop();
    }

    while !display.rl.window_should_close() {
        pool.queue_missing_chunks(&display);
        pool.apply_ready_chunks(&mut display, &terr);

        display.draw_loop();
    }

    // cleanup threads before exiting
    pool.shutdown();
}

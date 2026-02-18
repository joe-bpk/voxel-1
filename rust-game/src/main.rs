mod display;
mod level;
use crate::{
    display::{Display, RENDER_DISTANCE},
    level::terrain::{Chunk, DynTerr},
};
use level::utils::*;
use std::collections::HashSet;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

const NUM_CHUNK_THREADS: usize = 4;

struct ChunkWorkerPool
{
    work_tx:   mpsc::Sender<Option<ChunkLoc>>,
    result_rx: mpsc::Receiver<Chunk>,
    handles:   Vec<thread::JoinHandle<()>>,
    pending:   HashSet<ChunkLoc>,
}

impl ChunkWorkerPool
{
    fn new(terr: Arc<Mutex<DynTerr>>) -> Self
    {
        let (work_tx, work_rx) = mpsc::channel::<Option<ChunkLoc>>();
        let (result_tx, result_rx) = mpsc::channel::<Chunk>();
        let work_rx = Arc::new(Mutex::new(work_rx));

        let handles = (0..NUM_CHUNK_THREADS)
            .map(|_| {
                let work_rx = Arc::clone(&work_rx);
                let result_tx = result_tx.clone();
                let terr = Arc::clone(&terr);

                thread::spawn(move || {
                    loop {
                        let job = work_rx.lock().unwrap().recv().unwrap();
                        match job {
                            None => break,
                            Some(pos) => {
                                let chunk = terr
                                    .lock()
                                    .unwrap()
                                    .get_chunk(pos)
                                    .unwrap();
                                result_tx.send(chunk).unwrap();
                            }
                        }
                    }
                })
            })
            .collect();

        ChunkWorkerPool {
            work_tx,
            result_rx,
            handles,
            pending: HashSet::new(),
        }
    }

    fn queue_missing_chunks(&mut self, display: &Display)
    {
        for x in 0..RENDER_DISTANCE as i32 {
            for z in 0..RENDER_DISTANCE as i32 {
                for (sx, sz) in [(1, 1), (-1, 1), (1, -1), (-1, -1)] {
                    let pos = ChunkLoc {
                        loc: IntVec3 {
                            x: x * sx,
                            y: 0,
                            z: z * sz,
                        },
                    };

                    if display.is_chunk_loaded(pos)
                        || self.pending.contains(&pos)
                    {
                        continue;
                    }
                    self.pending.insert(pos);
                    self.work_tx.send(Some(pos)).unwrap();
                }
            }
        }
    }

    fn apply_ready_chunks(&mut self, display: &mut Display)
    {
        while let Ok(chunk) = self.result_rx.try_recv() {
            let pos = chunk.chunk_loc;
            self.pending.remove(&pos);
            display.load_chunk(&chunk);
        }
    }

    fn shutdown(self)
    {
        for _ in &self.handles {
            self.work_tx.send(None).unwrap();
        }
        for h in self.handles {
            h.join().unwrap();
        }
    }
}

fn main()
{
    let mut display = display::Display::new();
    display.rl.set_target_fps(100);

    let terr = Arc::new(Mutex::new(DynTerr::new()));
    let mut pool = ChunkWorkerPool::new(Arc::clone(&terr));

    while !display.rl.window_should_close() {
        pool.queue_missing_chunks(&display);
        pool.apply_ready_chunks(&mut display);
        display.draw_loop();
    }

    pool.shutdown();
}

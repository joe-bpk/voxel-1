use crate::level::utils::*;
use crate::{
    display::{Display, RENDER_DISTANCE},
    level::terrain::{Chunk, DynTerr},
};
use std::collections::HashSet;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

const NUM_CHUNK_THREADS: usize = 1; // changing this to 1 fixed "lazy" chunk loading issue, where some close chunks were not loading before further ones

pub struct ChunkWorkerPool
{
    work_tx:   mpsc::Sender<Option<ChunkLoc>>,
    result_rx: mpsc::Receiver<Chunk>,
    handles:   Vec<thread::JoinHandle<()>>,
    pending:   HashSet<ChunkLoc>,
}

impl ChunkWorkerPool
{
    pub fn new(terr: Arc<Mutex<DynTerr>>) -> Self
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
                        // receive job from the shared receiver
                        let job = work_rx.lock().unwrap().recv().unwrap();
                        match job {
                            None => break, // shutdown signal
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

    pub fn queue_missing_chunks(&mut self, display: &Display)
    {
        // iterate through distances (rings) starting from 0 (player position)
        for d in 0..RENDER_DISTANCE as i32 {
            // iterate over the perimeter of the square at distance d
            for x in -d..=d {
                for z in -d..=d {
                    // only process the edge of the square to form a "ring"
                    // this skips the inner squares already processed in previous 'd' iterations
                    if x.abs() != d && z.abs() != d {
                        continue;
                    }

                    let pos = ChunkLoc {
                        loc: IntVec3 {
                            x,
                            y: 0,
                            z,
                        },
                    };

                    if display.is_chunk_loaded(pos)
                        || self.pending.contains(&pos)
                    {
                        continue;
                    }

                    self.pending.insert(pos);
                    self.work_tx.send(Some(pos)).unwrap();

                    // optional: return early after queuing a few chunks to keep the frame smooth
                    // if you queue hundreds at once, it can still cause a slight hitch
                }
            }
        }
    }

    pub fn apply_ready_chunks(&mut self, display: &mut Display)
    {
        // non-blocking pull of all finished chunks
        while let Ok(chunk) = self.result_rx.try_recv() {
            let pos = chunk.chunk_loc;
            self.pending.remove(&pos);
            display.load_chunk(&chunk);
        }
    }

    pub fn shutdown(self)
    {
        for _ in 0..self.handles.len() {
            self.work_tx.send(None).unwrap();
        }
        for h in self.handles {
            h.join().unwrap();
        }
    }
}

use crate::level::utils::*;
use crate::{
    display::{ RENDER_DISTANCE},
    display::Display,
    level::terrain::{Chunk, DynTerr},
};

use crate::display::mesh::mesh_gen::ChunkNeighbors;
use std::collections::HashSet;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

const NUM_CHUNK_THREADS: usize = 4; // changing this to 1 fixed "lazy" chunk loading issue, where some close chunks were not loading before further ones

pub struct ChunkWorkerPool {
    work_tx: mpsc::Sender<Option<ChunkLoc>>,
    result_rx: mpsc::Receiver<Chunk>,
    handles: Vec<thread::JoinHandle<()>>,
    pending: HashSet<ChunkLoc>,
}

impl ChunkWorkerPool {
    pub fn new(terr: Arc<Mutex<DynTerr>>) -> Self {
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
                                let chunk = terr.lock().unwrap().get_chunk(pos).unwrap();
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

    pub fn queue_missing_chunks(&mut self, display: &Display) {
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
                        loc: IntVec3 { x, y: 0, z },
                    };

                    if display.is_chunk_loaded(pos) || self.pending.contains(&pos) {
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

    pub fn apply_ready_chunks(&mut self, display: &mut Display, terr: &Arc<Mutex<DynTerr>>) {
        let guard = terr.lock().unwrap();

        // non-blocking pull of all finished chunks
        while let Ok(chunk) = self.result_rx.try_recv() {
            let pos = chunk.chunk_loc;
            self.pending.remove(&pos);

            // helper to safely find a neighbor in the locked terrain data
            let get_neighbor = |dx: i32, dz: i32| -> Option<&Chunk> {
                let target = IntVec3 {
                    x: pos.loc.x + dx,
                    y: 0,
                    z: pos.loc.z + dz,
                };
                guard.chunks.iter().find(|c| c.chunk_loc.loc == target)
            };

            // 1. load the new chunk with its neighbors
            let neighbors = ChunkNeighbors {
                pos_x: get_neighbor(1, 0),
                neg_x: get_neighbor(-1, 0),
                pos_z: get_neighbor(0, 1),
                neg_z: get_neighbor(0, -1),
            };
            display.load_chunk(&chunk, &neighbors);

            // 2. refresh neighbors to cull their boundary faces against the new chunk
            let neighbor_offsets = [(1, 0), (-1, 0), (0, 1), (0, -1)];
            for (dx, dz) in neighbor_offsets {
                if let Some(neighbor_chunk) = get_neighbor(dx, dz) {
                    if display.is_chunk_loaded(neighbor_chunk.chunk_loc) {
                        let n_pos = neighbor_chunk.chunk_loc.loc;
                        let get_n_neighbor = |ndx: i32, ndz: i32| -> Option<&Chunk> {
                            let target = IntVec3 {
                                x: n_pos.x + ndx,
                                y: 0,
                                z: n_pos.z + ndz,
                            };
                            guard.chunks.iter().find(|c| c.chunk_loc.loc == target)
                        };

                        let n_neighbors = ChunkNeighbors {
                            pos_x: get_n_neighbor(1, 0),
                            neg_x: get_n_neighbor(-1, 0),
                            pos_z: get_n_neighbor(0, 1),
                            neg_z: get_n_neighbor(0, -1),
                        };
                        display.load_chunk(neighbor_chunk, &n_neighbors);
                    }
                }
            }
        }
    }

    pub fn shutdown(self) {
        for _ in 0..self.handles.len() {
            self.work_tx.send(None).unwrap();
        }
        for h in self.handles {
            h.join().unwrap();
        }
    }
}

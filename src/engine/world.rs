use std::cell::UnsafeCell;
use std::ops::Deref;
use std::sync::{Arc, Barrier, RwLock};
use std::thread;
use rsevents_extra::Semaphore;
use sfml::graphics::{CircleShape, Color, RenderTarget, RenderWindow, Shape, Transformable};
use sfml::system::Vector2f;
use crate::engine::cell_matrix::{CellMatrix, ThreadEngineObject, ThreadEngineObjectPtr};
use crate::engine::circle::Circle;
use crate::engine::engine_object::EngineObject;
use crate::utils::Vec2f;

const GRAVITY: Vec2f = Vec2f { x: 0.0, y: 1000.0 };
const THREAD_CNT: u32 = 12;
const CELL_SIZE: u32 = 32;

pub struct World {
    world_center: Vec2f,
    world_radius: f64,
    objects: Vec<ThreadEngineObjectPtr>,
    sub_step_cnt: u32,

    // Cell related variables
    cell_matrix: Arc<RwLock<CellMatrix>>,
    // cell_matrix: Arc<RwLock<Vec<Vec<Vec<Arc<Box<dyn EngineObject + Send + Sync>>>>>>>,
    // cell_matrix_width: u32,
    // cell_matrix_height: u32,

    // Threading-related variables
    start_sem: Arc<Semaphore>,
    end_barrier: Arc<Barrier>,
}

impl World {
    pub fn new(center: Vec2f, radius: f64) -> Self {
        // Create Arcs
        let start_sem = Arc::new(Semaphore::new(0, THREAD_CNT as u16));
        let worker_barrier = Arc::new(Barrier::new(THREAD_CNT as usize));
        let end_barrier = Arc::new(Barrier::new(THREAD_CNT as usize + 1));

        // Create cell matrix
        let matrix_w = ((center.x as f64 + radius) / CELL_SIZE as f64).ceil() as u32;
        let matrix_h = ((center.y as f64 + radius) / CELL_SIZE as f64).ceil() as u32;
        let cell_matrix = Arc::new(RwLock::new(
            CellMatrix::new(matrix_w, matrix_h, CELL_SIZE)
        ));
        // let cell_matrix = Arc::new(RwLock::new(
        //     vec![vec![Vec::new(); cell_matrix_width as usize]; cell_matrix_height as usize]
        // ));


        // Spawn worker threads
        let rows_per_thread = matrix_h / THREAD_CNT;
        let mut remainder = matrix_h % THREAD_CNT;
        let extra_barrier = Arc::new(Barrier::new(remainder as usize));
        let mut start_index: u32 = 0;
        for i in 0..THREAD_CNT {
            let start_sem = start_sem.clone();
            let end_barrier = end_barrier.clone();
            let worker_barrier = worker_barrier.clone();
            let extra_barrier = extra_barrier.clone();
            let cell_matrix = cell_matrix.clone();

            let worker = WorkerThread {
                id: i,
                cell_matrix,
                start_index,
                row_cnt: rows_per_thread,
                extra_pass: remainder > 0,
                start_sem,
                end_barrier,
                worker_barrier,
                extra_barrier,
            };

            start_index += worker.row_cnt + worker.extra_pass as u32;
            if remainder > 0 { remainder -= 1; }

            thread::spawn(move || { worker.logic(); });
        }


        World {
            world_center: center,
            world_radius: radius,
            objects: Vec::new(),
            sub_step_cnt: 1,
            cell_matrix,
            start_sem,
            end_barrier,
        }
    }

    pub fn sfml_render(&mut self, window: &mut RenderWindow) {
        // Clear window
        window.clear(Color {
            r: 32,
            g: 32,
            b: 32,
            a: 255,
        });

        // Draw world boundary
        let mut world_boundary = CircleShape::new(1.0, 64);
        // Set origin to middle of 2x2 circle (r=1), then scale to proper size
        world_boundary.set_origin(Vector2f {
            x: 1.0,
            y: 1.0,
        });
        world_boundary.set_position(Vector2f {
            x: self.world_center.x as f32,
            y: self.world_center.y as f32,
        });
        world_boundary.set_scale(Vector2f {
            x: self.world_radius as f32,
            y: self.world_radius as f32,
        });
        world_boundary.set_fill_color(Color {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        });
        window.draw(&world_boundary);

        // Draw cell matrix boundaries
        // let m_h = self.cell_matrix.read().unwrap().get_height();
        // let m_w = self.cell_matrix.read().unwrap().get_width();
        // let cell_width = 912 / m_w;
        // let cell_height = 912 / m_h;
        // for row in 0..m_h {
        //     for col in 0..m_w {
        //         let mut rect = RectangleShape::with_size(Vector2f::from((5.0, 1024.0)));
        //         rect.set_position(Vector2f::from(((cell_width * col) as f32, 0.0)));
        //         rect.set_fill_color(Color::rgb(64, 32, 64));
        //         window.draw(&rect);
        //
        //     }
        // }

        for obj in &self.objects {
            if let Some(circle) = obj.read().unwrap().deref().as_any().downcast_ref::<Circle>() {
                let mut shape = CircleShape::new(1.0, 32);

                shape.set_origin(Vector2f {
                    x: 1.0,
                    y: 1.0,
                });
                // let mut shape = CircleShape::new(1.0, 32);
                shape.set_position(Vector2f {
                    x: circle.get_pos().x as f32,
                    y: circle.get_pos().y as f32,
                });
                shape.set_scale(Vector2f {
                    x: circle.get_radius() as f32,
                    y: circle.get_radius() as f32,
                });

                shape.set_fill_color(Color {
                    r: 255,
                    g: 255,
                    b: 255,
                    a: 255,
                });
                window.draw(&shape);
            }
        }
    }

    pub fn add_object(&mut self, object: Box<ThreadEngineObject>) {
        let arc_obj = Arc::new(RwLock::new(object));
        self.objects.push(arc_obj.clone());
        let idx = self.objects.len() - 1;

        // From pos, get row and col in cell matrix
        let obj = self.objects[idx].read().unwrap();
        let row_idx = (obj.get_pos().x as u32 / CELL_SIZE) as usize;
        let col_idx = (obj.get_pos().x as u32 / CELL_SIZE) as usize;
        self.cell_matrix.write().unwrap().add_object_to_cell(row_idx, col_idx, arc_obj);
    }

    pub fn set_sub_step_cnt(&mut self, cnt: u32) {
        self.sub_step_cnt = cnt;
    }

    pub fn tick(&mut self, dt: f64) {
        let step_dt = dt / self.sub_step_cnt as f64;

        for _ in 0..self.sub_step_cnt {
            self.apply_gravity();
            // self.check_collisions(step_dt);
            self.apply_constraints();
            // Alert worker threads that they should check for collisions
            self.start_sem.release(THREAD_CNT as u16);
            self.end_barrier.wait();  // Wait until all worker threads are done

            for i in 0..self.objects.len() {
                self.objects[i].write().unwrap().tick(step_dt);
            }
            self.apply_constraints();
        }
    }

    fn apply_gravity(&mut self) {
        for obj in self.objects.iter_mut() {
            obj.write().unwrap().accelerate(&GRAVITY);
        }
    }

    // fn check_collisions(&mut self, _dt: f64) {
    //     for i in 0..self.objects.len() {
    //         for j in i + 1..self.objects.len() {
    //             let a = self.objects[i].read().unwrap();
    //             let b = self.objects[j].read().unwrap();
    //             let vec = a.get_pos() - b.get_pos();
    //             let dist_sq = vec.get_magnitude_squared();
    //             let min_dist = a.min_dist(b.as_ref()) + b.min_dist(a.as_ref());
    //             // Check if distance is larger than some number to avoid moving cells with very
    //             // slight overlap
    //             if dist_sq > 0.0001 && dist_sq < min_dist * min_dist {
    //                 // Collision, move objects
    //                 let dist = dist_sq.sqrt();
    //                 // Get unit vector to use for direction
    //                 let unit_vec = vec / dist;
    //                 // Get ratio of masses to calculate movement
    //                 let r1 = a.get_mass() / (a.get_mass() + b.get_mass());
    //                 let r2 = b.get_mass() / (a.get_mass() + b.get_mass());
    //                 // Get half the movement needed, if both objects move by half, full movement done
    //                 let half_delta = 0.5 * 0.75 * (dist - min_dist);
    //
    //                 self.objects[i].write().unwrap().sub_pos(&(unit_vec * half_delta * r1));
    //                 self.objects[j].write().unwrap().add_pos(&(unit_vec * half_delta * r2));
    //             }
    //         }
    //     }
    // }

    fn apply_constraints(&mut self) {
        for obj in &self.objects {
            obj.write().unwrap().constraint(&self.world_center, self.world_radius);
        }
    }
}

// -----
// WORKER THREAD
// -----

struct WorkerThread {
    id: u32,
    cell_matrix: Arc<RwLock<CellMatrix>>,
    start_index: u32,
    row_cnt: u32,
    extra_pass: bool,
    start_sem: Arc<Semaphore>,
    self_sem: Arc<Semaphore>,
    next_sem: Arc<Semaphore>,
    end_barrier: Arc<Barrier>,
    worker_barrier: Arc<Barrier>,
    extra_barrier: Arc<Barrier>,
}

impl WorkerThread {
    pub fn logic(&self) {
        let matrix_w = self.cell_matrix.read().unwrap().get_width();
        let row_cnt = if self.extra_pass { self.row_cnt + 1 } else { self.row_cnt };

        loop {
            // Wait until new tick
            debug!("[{}] Waiting for tick", self.id);
            self.start_sem.wait().forget();
            // Don't auto re-increment sem counter once sem_guard drops
            debug!("[{}] Got tick", self.id);

            for cell_row in self.start_index..self.start_index + row_cnt {
                debug!("[{}] Starting pass", self.id);
                for cell_col in 0..matrix_w {
                    let cell_matrix = self.cell_matrix.read().unwrap();
                    // Can write-lock the objects as we should be the only thread trying to access it
                    for a in cell_matrix.get_objects_in_cell(cell_row as usize, cell_col as usize) {
                        for b in cell_matrix.get_objects_in_neighbouring_cells(cell_row as usize, cell_col as usize) {
                            if Arc::ptr_eq(&a, &b) { continue; }  // Same refs, skip

                            // if &a as *const _ == &b as *const _ { continue; }  // Same refs, skip
                            let mut a = a.write().unwrap();
                            debug!("[{}] Acq a", self.id);
                            let mut b = b.write().unwrap();
                            debug!("[{}] Acq b", self.id);

                            // Valid pair, detect collision and fix if needed
                            let vec = a.get_pos() - b.get_pos();
                            let dist_sq = vec.get_magnitude_squared();
                            let min_dist = a.min_dist(b.as_ref()) + b.min_dist(a.as_ref());

                            // Check if distance is larger than some number to avoid moving cells with very
                            // slight overlap
                            if dist_sq > 0.0001 && dist_sq < min_dist * min_dist {
                                // Collision, move objects
                                let dist = dist_sq.sqrt();
                                // Get unit vector to use for direction
                                let unit_vec = vec / dist;
                                // Get ratio of masses to calculate movement
                                let r1 = a.get_mass() / (a.get_mass() + b.get_mass());
                                let r2 = b.get_mass() / (a.get_mass() + b.get_mass());
                                // Get half the movement needed, if both objects move by half, full movement done
                                let half_delta = 0.5 * 0.75 * (dist - min_dist);

                                a.sub_pos(&(unit_vec * half_delta * r1));
                                b.add_pos(&(unit_vec * half_delta * r2));
                            }
                        }
                    }
                }

                if self.extra_pass && cell_row + 1 == self.start_index + row_cnt {
                    // Extra pass has less threads to wait on, use appropriate barrier
                    let res = self.extra_barrier.wait();
                    if res.is_leader() {
                        // Update cell matrix (1 thread only)
                        debug!("[{}] Updating matrix", self.id);
                        self.cell_matrix.write().unwrap().update_positions();
                        debug!("[{}] Done updating", self.id);
                    }
                    // Wait until update is done
                    self.extra_barrier.wait();
                } else {
                    // Regular pass, use regular barrier
                    let res = self.worker_barrier.wait();
                    if res.is_leader() {
                        // Update cell matrix (1 thread only)
                        debug!("[{}] Updating matrix", self.id);
                        self.cell_matrix.write().unwrap().update_positions();
                        debug!("[{}] Done updating", self.id);
                    }
                    self.worker_barrier.wait();
                }
            }

            // Join barrier whilst some threads are doing their 1 extra pass
            // if !self.extra_pass {
            //     self.worker_barrier.wait();
            // }

            // Wait until all threads are done with all their passes and signal to main thread
            debug!("[{}] Tick finished", self.id);
            self.end_barrier.wait();
        }
    }
}

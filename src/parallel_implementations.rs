#![allow(unused)] // compiler doesn't complain about unused variables and mut anymore -> remove for release

pub mod parallel_implementations {
    use super::*;
    // use ascon_hash_implementation::*; // will only really be needing state

    use crate::*;

    use std::{
        sync::{mpsc, Arc, Barrier, Mutex}, // NOT FULLY IMPLEMENTED
        thread,
    };

    pub struct ThreadPool {
        workers: Vec<Worker>,
        sender: Option<mpsc::Sender<Job>>,
    }

    type Job = Box<dyn FnOnce() -> u64 + Send + 'static>;

    impl ThreadPool {
        pub fn new(size: usize) -> ThreadPool {
            assert!(size > 0);

            let (sender, receiver) = mpsc::channel();

            let receiver = Arc::new(Mutex::new(receiver));

            let mut workers = Vec::with_capacity(size);

            for id in 0..size {
                workers.push(Worker::new(id, Arc::clone(&receiver)));
            }

            ThreadPool {
                workers,
                sender: Some(sender),
            }
        }

        pub fn execute<F>(&self, f: F) -> mpsc::Receiver<u64>
        where
            F: FnOnce() -> u64 + Send + 'static,
        {
            let (result_sender, result_receiver) = mpsc::channel();

            let job: Job = Box::new(move || {
                let result = f();
                result // Return the result directly.
            });

            self.sender.as_ref().unwrap().send(job).unwrap();
            result_receiver
        }

        pub fn rotate_x0(&self, mut x_0: u64) -> mpsc::Receiver<u64> {
            self.execute(move || {
                x_0 ^= x_0.rotate_right(19) ^ x_0.rotate_right(28);
                x_0 // Return the result of rotate_xo.
            })
        }

        pub fn rotate_x1(&self, mut x_1: u64) -> mpsc::Receiver<u64> {
            self.execute(move || {
                x_1 ^= x_1.rotate_right(61) ^ x_1.rotate_right(39);

                x_1 // Return the result of rotate_xo.
            })
        }

        pub fn rotate_x2(&self, mut x_2: u64) -> mpsc::Receiver<u64> {
            self.execute(move || {
                x_2 ^= x_2.rotate_right(1) ^ x_2.rotate_right(6);
                x_2 // Return the result of rotate_xo.
            })
        }

        pub fn rotate_x3(&self, mut x_3: u64) -> mpsc::Receiver<u64> {
            self.execute(move || {
                x_3 ^= x_3.rotate_right(10) ^ x_3.rotate_right(17);
                x_3 // Return the result of rotate_xo.
            })
        }

        pub fn rotate_x4(&self, mut x_4: u64) -> mpsc::Receiver<u64> {
            self.execute(move || {
                x_4 ^= x_4.rotate_right(7) ^ x_4.rotate_right(41);

                x_4 // Return the result of rotate_xo.
            })
        }
    }

    impl Drop for ThreadPool {
        fn drop(&mut self) {
            drop(self.sender.take());

            for worker in &mut self.workers {
                println!("Shutting down worker {}", worker.id);

                if let Some(thread) = worker.thread.take() {
                    thread.join().unwrap();
                }
            }
        }
    }

    struct Worker {
        id: usize,
        thread: Option<thread::JoinHandle<()>>,
    }

    impl Worker {
        fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
            let thread = thread::spawn(move || loop {
                let message = receiver.lock().unwrap().recv();

                match message {
                    Ok(job) => {
                        job(); // Call the job to execute the closure.
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            });

            Worker {
                id,
                thread: Some(thread),
            }
        }
    }

    //////////////////// Ascon Implementations with thread pool ///////////////////////////

    // additional methods on the state
    impl State {
        pub fn single_permutation_pool(self, pool: &ThreadPool, round_constant: u64) -> Self {
            // round constant added further below
            // operations on single variable faster compared to arrays or even tuples
            let mut x_0 = self.x[0];
            let mut x_1 = self.x[1];
            let mut x_2 = self.x[2];
            let mut x_3 = self.x[3];
            let mut x_4 = self.x[4];

            // S-box
            x_0 ^= x_4;
            x_2 ^= x_1 ^ round_constant;
            x_4 ^= x_3; // -> no to paralel

            // intermediate variables with x
            let x_0_0: u64 = x_0; // should be a copy not a borrow
            let x_1_1: u64 = x_1;

            x_0 ^= (!x_1) & x_2;
            x_1 ^= (!x_2) & x_3;
            x_2 ^= (!x_3) & x_4;
            x_3 ^= (!x_4) & x_0_0;
            x_4 ^= (!x_0_0) & x_1_1;

            x_1 ^= x_0;
            x_3 ^= x_2;
            x_0 ^= x_4;

            x_2 = !x_2; // can be combined in the following linear layer by inverting the output

            // linear layer
            // maybe better to seperate

            // x_0 ^= x_0.rotate_right(19) ^ x_0.rotate_right(28);

            // Function to be executed in parallel to calculate new_x_0.
            // let calculate_new_x_0 = move || {
            //     let mut new_x_0 = x_0;
            //     new_x_0 ^= new_x_0.rotate_right(19) ^ new_x_0.rotate_right(28);
            //     new_x_0
            // };

            let x_0_receiver = pool.rotate_x0(x_0);

            let x_1_receiver = pool.rotate_x1(x_1);

            let x_2_receiver = pool.rotate_x2(x_2);

            let x_3_receiver = pool.rotate_x3(x_3);

            let x_4_receiver = pool.rotate_x4(x_4);

            let x_0 = x_0_receiver.recv().unwrap();
            let x_1 = x_1_receiver.recv().unwrap();
            let x_2 = x_2_receiver.recv().unwrap();
            let x_3 = x_3_receiver.recv().unwrap();
            let x_4 = x_4_receiver.recv().unwrap();
            // Calculate new x_0 using the thread pool.
            // let new_x_0 = self.thread_pool.execute(calculate_new_x_0).recv().unwrap();

            // return self
            State::new(x_0, x_1, x_2, x_3, x_4)
        }
        pub fn permutation_12_pool(mut self, pool: &ThreadPool) -> Self {
            // see above just with the single permutation function with outside variables
            self = self
                .single_permutation_pool(&pool, 0xf0)
                .single_permutation_pool(&pool, 0xe1)
                .single_permutation_pool(&pool, 0xd2)
                .single_permutation_pool(&pool, 0xc3)
                .single_permutation_pool(&pool, 0xb4)
                .single_permutation_pool(&pool, 0xa5)
                .single_permutation_pool(&pool, 0x96)
                .single_permutation_pool(&pool, 0x87)
                .single_permutation_pool(&pool, 0x78)
                .single_permutation_pool(&pool, 0x69)
                .single_permutation_pool(&pool, 0x5a)
                .single_permutation_pool(&pool, 0x4b);

            // return
            self
        }
    }

    pub fn ascon_hash_pool(input_string: &String) -> Vec<u64> {
        // padded input message divided into blocks
        let message_blocks = convert_pad_into_blocks(&input_string);

        // Initialization
        //let mut s : State = (State::new(0x00400c0000000100, 0, 0, 0, 0)).permutation_12();

        // Initalized state precomputed
        let mut s: State = State::new(
            0xee9398aadb67f03d,
            0x8bb21831c60f1002,
            0xb48a92db98d5da62,
            0x43189921b8f8e3e8,
            0x348fa5c9d525e140,
        );
        let pool = ThreadPool::new(5);

        // Absorption
        for i in &message_blocks {
            s.x[0] ^= *i;
            s = s.permutation_12_pool(&pool); // very last implementation is part of the Squeezing Phase
        }

        // Squeezing
        let mut output_blocks: Vec<u64> = Vec::new();

        // output of Hash is 256 bits = 64*4 -> 4 blocks
        for _ in 0..3 {
            output_blocks.push(s.x[0]);
            s = s.permutation_12_pool(&pool);
        }
        output_blocks.push(s.x[0]);
        output_blocks
    }
} // end of mod
  //

// mod async_implementation {
//     use super::*;
//     use crate::State;
//     use async_trait::async_trait;
//     use tokio::task;
// }

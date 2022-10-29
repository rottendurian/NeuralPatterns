pub mod shader;
pub mod debug;
pub mod camera;
pub extern crate glfw;
pub extern crate glam;
pub mod gl {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub mod timer {
    use std::time::{Instant};


    pub struct Time {
        pub prev_time:Instant,
        pub count:f64,
        pub fps:f64
    }

    impl Time {
        pub fn new() -> Time {
            Time {
                prev_time:Instant::now(),
                count:0.0,
                fps:0.0
            }
        }
        
        pub fn reset_delta(&mut self) {
            self.prev_time = Instant::now();
        }

        pub fn get_delta(&self) -> f64 {
            self.prev_time.elapsed().as_secs_f64()
        }

        pub fn check_delta(&mut self, expected_milli:f64) -> bool {
            let delta_time = self.prev_time.elapsed().as_secs_f64();
            self.count+=1.0;
            if  delta_time > expected_milli {
                let delta = delta_time/self.count;                
                self.fps = 1.0/delta;
                self.count = 0.0;
                self.reset_delta();
                return true;
            }
            false
        }

    }

}




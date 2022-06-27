use std::{thread::sleep, time::Duration};

pub struct App {
    progress: bool,
    frame: Duration,
}

impl App {
    pub fn create() -> App {
        App {
            progress: true,
            frame: Duration::from_secs_f32(1.0 / 30.0),
        }
    }

    pub fn execute(&self) {
        while self.progress {
            self.update();
            sleep(self.frame);
        }
    }

    fn update(&self) {
        // TODO : Make update
        println!("UPDATED");
    }
}

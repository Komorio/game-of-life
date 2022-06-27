use std::{thread::sleep, time::Duration};

pub struct App {
    progress: bool,
    frame: Duration,
    index: u8,
}

impl App {
    pub fn create() -> App {
        App {
            progress: true,
            frame: Duration::from_secs_f32(1.0 / 30.0),
            index: 0,
        }
    }

    pub fn execute(&mut self) {
        while self.progress {
            self.update();
            self.render();
            sleep(self.frame);
            self.release_render();
        }
    }

    fn update(&mut self) {
        self.index += 1;
    }

    fn render(&self) {
        println!("{}", self.index);
    }

    fn release_render(&self) {
        std::process::Command::new("clear")
            .status()
            .expect("Fail to clear screen.");
    }
}

use std::{thread::sleep, time::Duration};

const MAP_WIDTH: u8 = 40;
const MAP_HEIGHT: u8 = 15;

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

pub struct Vector2 {
    pub x: u8,
    pub y: u8,
}

impl Vector2 {
    pub fn new() -> Vector2 {
        Vector2 { x: 0, y: 0 }
    }
    pub fn from(x: u8, y: u8) -> Vector2 {
        Vector2 { x, y }
    }
}

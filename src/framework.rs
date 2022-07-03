use std::{thread::sleep, time::Duration};

const MAP_WIDTH: u8 = 40;
const MAP_HEIGHT: u8 = 15;

pub struct App {
    progress: bool,
    frame: Duration,
    map: Vec<Vec<Cell>>,
}

impl App {
    pub fn create() -> App {
        App {
            progress: true,
            frame: Duration::from_secs_f32(1.0 / 30.0),
            map: App::create_map(),
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

    fn update(&mut self) {}

    fn render(&self) {
        for y in 0..(MAP_HEIGHT as usize) {
            let mut x_str = String::new();

            for x in 0..(MAP_WIDTH as usize) {
                x_str.push(self.map[y][x].character);
            }

            println!("{}", x_str);
        }
    }

    fn release_render(&self) {
        std::process::Command::new("clear")
            .status()
            .expect("Fail to clear screen.");
    }

    fn create_map() -> Vec<Vec<Cell>> {
        let mut map: Vec<Vec<Cell>> = vec![];

        for y in 0..MAP_HEIGHT {
            let mut x_map: Vec<Cell> = vec![];

            for x in 0..MAP_WIDTH {
                x_map.push(Cell {
                    character: '#',
                    position: Vector2::from(x, y),
                });
            }

            map.push(x_map);
        }

        map
    }
}

pub struct Cell {
    pub character: char,
    pub position: Vector2,
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

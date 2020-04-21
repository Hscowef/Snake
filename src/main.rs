extern crate minifb;

use std::time::SystemTime;

use minifb::{Key, Window, WindowOptions};
use rand::Rng;

const SQUARE_SIZE: usize = 25;
const GRID_DIMENSIONS: usize = 25;

const UPDATE_TIME: u128 = 200;

const FRAME_DIMENSION: usize = SQUARE_SIZE * GRID_DIMENSIONS;

fn main() {
    let mut game = SnakeGame::new();
    game.play()

    
}

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None
}

struct SnakeGame {
    snake_position: Vec<usize>,
    snake_direction: Direction,
    food: usize,
    dead: bool,
    windows_buffer: Vec<u32>,
    move_clock: SystemTime
}

impl SnakeGame {
    pub fn new() -> SnakeGame {
        SnakeGame {
            snake_position: vec![rand::thread_rng().gen_range(0, GRID_DIMENSIONS * GRID_DIMENSIONS)],
            snake_direction: Direction::None,
            food: rand::thread_rng().gen_range(0, GRID_DIMENSIONS * GRID_DIMENSIONS),
            dead: false,
            windows_buffer: vec![(255 << 16) | (255 << 8) | 255; FRAME_DIMENSION * FRAME_DIMENSION],
            move_clock: SystemTime::now()
        }
    }

    pub fn play(&mut self) {
        let mut window = Window::new(
            "Snake",
            FRAME_DIMENSION,
            FRAME_DIMENSION,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });
            
        while window.is_open() && !window.is_key_down(Key::Escape) {
            if self.dead {
                println!("You lost, press enter to play again.");
                loop {
                    if window.is_key_down(Key::Enter) {
                        self.reset();
                        break
                    }

                    window
                        .update_with_buffer(&self.windows_buffer, FRAME_DIMENSION, FRAME_DIMENSION)
                        .unwrap();
                }

            }

            if window.is_key_down(Key::Up) {
                if self.snake_direction != Direction::Down {
                    self.snake_direction = Direction::Up
                }

            } else if window.is_key_down(Key::Down) {
                if self.snake_direction != Direction::Up {
                    self.snake_direction = Direction::Down
                }
            } else if window.is_key_down(Key::Left) {
                if self.snake_direction != Direction::Right {
                    self.snake_direction = Direction::Left
                }
            } else if window.is_key_down(Key::Right) {
                if self.snake_direction != Direction::Left {
                    self.snake_direction = Direction::Right
                }
            }
            
            if self.move_clock.elapsed().unwrap().as_millis() >= UPDATE_TIME {
                self.move_snake();
                self.move_clock = SystemTime::now()
            }

            self.update_buffer();
            window
                .update_with_buffer(&self.windows_buffer, FRAME_DIMENSION, FRAME_DIMENSION)
                .unwrap();
        }
    }

    fn reset(&mut self) {
        let mut _new_game = Self::new();
        
        self.snake_position = _new_game.snake_position;
        self.snake_direction = _new_game.snake_direction;
        self.food = _new_game.food;
        self.dead = _new_game.dead;
        self.windows_buffer = _new_game.windows_buffer;
        self.move_clock = _new_game.move_clock;
    }

    fn update_buffer(&mut self) {
        let mut buffer: Vec<u32> = vec![];

        for rows in 0..GRID_DIMENSIONS {
            let mut r = vec![]; 

            for val in 0..GRID_DIMENSIONS {
                for _ in 0..SQUARE_SIZE {
                    let i = rows * GRID_DIMENSIONS + val;
                    if self.snake_position.iter().any(|&v| v == i) {
                        r.push(0)
                    } else if self.food == i {
                        r.push((255 << 16) | (0 << 8) | 0)
                    } else {
                        r.push((255 << 16) | (255 << 8) | 255)
                    }
                }

            }

            for _ in 0..SQUARE_SIZE {
                buffer.extend(r.iter())
            }
        }
        self.windows_buffer = buffer;
    }

    fn create_food(&mut self) {
        loop {
            let mut in_snake = false;
            let random_nbr = rand::thread_rng().gen_range(0, GRID_DIMENSIONS * GRID_DIMENSIONS);
            for pos in self.snake_position.iter() {
                if pos == &random_nbr {
                    in_snake = true
                }
            }
            if !in_snake {
                self.food = random_nbr;
                return;
            }
        }   
    }

    fn move_snake(&mut self) {

        match self.snake_direction {
            Direction::Up => {
                let in_first_row = self.snake_position[0] < GRID_DIMENSIONS;
                if in_first_row {
                    self.dead = true;
                    return
                }
                
                self.snake_position.insert(0, self.snake_position[0] - GRID_DIMENSIONS);
            },
            Direction::Down => {
                let in_last_row = self.snake_position[0] >= (GRID_DIMENSIONS * GRID_DIMENSIONS) - GRID_DIMENSIONS;
                if in_last_row {
                    self.dead = true;
                    return
                }
                
                self.snake_position.insert(0, self.snake_position[0] + GRID_DIMENSIONS);
            }
            Direction::Left => {
                let in_left_column = self.snake_position[0] % GRID_DIMENSIONS == 0;
                if in_left_column {
                    self.dead = true;
                    return
                }
                
                self.snake_position.insert(0, self.snake_position[0] - 1);
            }

            Direction::Right => {
                let in_right_column = (self.snake_position[0] + 1) % GRID_DIMENSIONS == 0;
                if in_right_column {
                    self.dead = true;
                    return
                }
                
                self.snake_position.insert(0, self.snake_position[0] + 1);
            },
            Direction::None => return
        }
        if self.ate_food() {
            self.create_food();
        } else if self.ate_himself() {
            self.dead = true
        } else {
            self.snake_position.pop();
        }
    }

    fn ate_food(&self) -> bool {
        self.snake_position[0] == self.food
    }

    fn ate_himself(&self) -> bool {
        let mut snake = self.snake_position.clone();
        snake.remove(0);
        snake.iter().any(|&v| v == self.snake_position[0])
    }
}
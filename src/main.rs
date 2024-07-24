use std::cmp::PartialEq;

use device_query::{DeviceQuery, DeviceState, Keycode};
use rand::Rng;

struct Point {
    x: u32,
    y: u32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

enum DIR {
    Up,
    Down,
    Left,
    Right,
}

fn is_key_down(key: &Keycode) -> bool {
    let device_state = DeviceState::new();
    let keys: Vec<Keycode> = device_state.get_keys();
    return keys.contains(key);
}

fn check_for_input(dir: DIR) -> DIR {
    if is_key_down(&Keycode::Up) {
        return DIR::Up;
    } else if is_key_down(&Keycode::Down) {
        return DIR::Down;
    } else if is_key_down(&Keycode::Left) {
        return DIR::Left;
    } else if is_key_down(&Keycode::Right) {
        return DIR::Right;
    } else {
        return dir;
    }
}

fn generate_food(food: &mut Point, width: &u32, height: &u32) {
    let mut rng = rand::thread_rng();
    food.x = rng.gen_range(0..*height);
    food.y = rng.gen_range(0..*width);
}

fn get_valid_point(inp_point: Point, width: &u32, height: &u32) -> Point {
    fn get_valid_val(val: u32, bound: &u32) -> u32 {
        if val >= *bound {
            return val - bound;
        } else {
            return val;
        }
    }

    return Point {
        x: get_valid_val(inp_point.x, height),
        y: get_valid_val(inp_point.y, width),
    };
}

fn progress_game(
    snake: &mut Vec<Point>,
    dir: &DIR,
    food: &mut Point,
    width: &u32,
    height: &u32,
) -> bool {
    let new_pos = match dir {
        DIR::Up => get_valid_point(
            Point {
                x: if snake[0].x > 0 {
                    snake[0].x - 1
                } else {
                    height.clone() - 1
                },
                y: snake[0].y,
            },
            width,
            height,
        ),
        DIR::Down => get_valid_point(
            Point {
                x: snake[0].x + 1,
                y: snake[0].y,
            },
            width,
            height,
        ),
        DIR::Left => get_valid_point(
            Point {
                x: snake[0].x,
                y: if snake[0].y > 0 {
                    snake[0].y - 1
                } else {
                    width.clone() - 1
                },
            },
            width,
            height,
        ),
        DIR::Right => get_valid_point(
            Point {
                x: snake[0].x,
                y: snake[0].y + 1,
            },
            width,
            height,
        ),
    };

    if new_pos == *food {
        generate_food(food, width, height);
    } else {
        _ = snake.pop();
    }
    let ret_val: bool = !snake.contains(&new_pos);
    snake.insert(0, new_pos);
    return ret_val;
}

fn render_game(snake: &Vec<Point>, food: &Point, width: &u32, height: &u32) {
    for i in 0..*height {
        for j in 0..*width {
            let po = Point { x: i, y: j };
            if snake.contains(&po) {
                print!("#");
            } else if po == *food {
                print!("*");
            } else {
                print!("-");
            }
        }
        println!();
    }
}

fn main() {
    let width: u32 = 30;
    let height: u32 = 10;

    let mut food: Point = Point { x: 0, y: 0 };
    let mut is_game_running: bool = true;
    let mut dir: DIR = DIR::Up;

    let mut snake: Vec<Point> = vec![Point {
        x: height / 2,
        y: width / 2,
    }];

    generate_food(&mut food, &width, &height);

    while is_game_running {
        dir = check_for_input(dir);
        is_game_running = progress_game(&mut snake, &dir, &mut food, &width, &height);

        println!("-*The Snake Game.*-");
        render_game(&snake, &food, &width, &height);

        std::thread::sleep(std::time::Duration::from_millis(100));

        if is_game_running {
            clearscreen::clear().unwrap();
        } else {
            println!("Game Over !!!");
        }
    }
}

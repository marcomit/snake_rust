use macroquad::prelude::*;
// use std::{time, thread};


const BLOCK: u8 = 20;
const WIDTH: u8 = 20;
const HEIGHT: u8 = 20;
fn setup() -> Conf {
    Conf {
        window_resizable: false,
        ..Default::default()
    }
}

type Point = (i16, i16);
#[derive(Clone)]
struct Snake {
    tails: Vec<Point>,
    direction: Point
}

#[macroquad::main(setup)]
async fn main() {
    let mut score = 0;
    let mut snake: Snake = Snake {tails: Vec::new(), direction: (1, 0)};
    let mut apple: Point = (12, 12);
    snake.tails.push((2, 10));
    let mut last_update = get_time();
    let speed = 0.2;
    loop {
        apply_direction(&mut snake);
        clear_background(BLACK);
        let label = String::from(score.to_string());
        draw_text(&label, 20.0, 20.0, 20.0, Color::from_rgba(255, 255, 255, 255));
        draw_rectangle((apple.0 as f32) * (BLOCK as f32), (apple.1 as f32) * (BLOCK as f32), BLOCK.into(), BLOCK.into(), Color::from_rgba(255, 0, 0, 255));
        for (x, y) in snake.tails.clone() {
            draw_rectangle((x as f32) * (BLOCK as f32), (y as f32) * (BLOCK as f32), BLOCK.into(), BLOCK.into(), Color::from_rgba(0, 255, 0, 255));
        }
        if check_wall(&snake) || check_self(&snake) { break; }

        while get_time() - last_update < speed.into() {apply_direction(&mut snake);}

        last_update = get_time();
        let (head_x, head_y) = snake.tails[0];
        let new_tail: Point = (head_x + snake.direction.0, head_y + snake.direction.1);
        snake.tails.pop();
        snake.tails.insert(0, new_tail);

        if snake.tails[0].0 == apple.0 && snake.tails[0].1 == apple.1 {
            score += 1;
            apple = (rand::gen_range(0, WIDTH.into()), rand::gen_range(0, HEIGHT.into()));
            let tail = snake.tails.last().unwrap();
            snake.tails.push(*tail);
        }
        next_frame().await;
    }

}

fn check_wall(snake: &Snake) -> bool {
    let (x, y) = snake.tails[0];
    !in_between(x, (-1, (WIDTH + 1).into())) || !in_between(y, (-1, (HEIGHT + 1).into()))
}

fn check_self(snake: &Snake) -> bool {
    let head = snake.tails[0];
    for i in 3..snake.tails.len() {
        if head == snake.tails[i] {
            return true;
        }
    }
    false
}

fn in_between(num: i16, range: Point) -> bool {
    let (min, max) = range;
    num >= min && num <= max
}

fn apply_direction(snake: &mut Snake) {
    if is_key_down(KeyCode::D) && snake.direction != (-1, 0) {
        snake.direction = (1, 0);
    }
    else if is_key_down(KeyCode::S) && snake.direction != (0, -1) {
        snake.direction = (0, 1);
    }
    else if is_key_down(KeyCode::A) && snake.direction != (1, 0) {
        snake.direction = (-1, 0);
    }
    else if is_key_down(KeyCode::W) && snake.direction != (0, -1) {
        snake.direction = (0, -1);
    }
}
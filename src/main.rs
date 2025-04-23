use macroquad::prelude::*;

use std::collections::LinkedList;

const SQUARES: i16 = 14;

type Point = (i16, i16);

struct Snake {
    body: LinkedList<Point>,
    head: Point,
    dir: Point,
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn vector(&self) -> Point {
        match self {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
        }
    }
}

struct SnakeGame {
    snake: Snake,
    fruit: Point,
    score: i32,
    speed: f64,
    last_update: f64,
    navigation_lock: bool,
    game_over: bool,
}

impl Default for SnakeGame {
    fn default() -> Self {
        Self {
            fruit: (rand::gen_range(0, SQUARES), rand::gen_range(0, SQUARES)),
            score: 0,
            speed: 0.3,
            last_update: get_time(),
            navigation_lock: false,
            game_over: false,
            snake: Snake {
                body: LinkedList::new(),
                head: (0, 0),
                dir: (1, 0),
            },
        }
    }
}

impl SnakeGame {
    fn handle_direction(&mut self) {
        let SnakeGame {
            snake,
            navigation_lock,
            ..
        } = self;
        if is_key_down(KeyCode::Right) && snake.dir != Direction::Left.vector() && !*navigation_lock
        {
            snake.dir = Direction::Right.vector();
            *navigation_lock = true;
        } else if is_key_down(KeyCode::Left)
            && snake.dir != Direction::Right.vector()
            && !*navigation_lock
        {
            snake.dir = Direction::Left.vector();
            *navigation_lock = true;
        } else if is_key_down(KeyCode::Up)
            && snake.dir != Direction::Down.vector()
            && !*navigation_lock
        {
            snake.dir = Direction::Up.vector();
            *navigation_lock = true;
        } else if is_key_down(KeyCode::Down)
            && snake.dir != Direction::Up.vector()
            && !*navigation_lock
        {
            snake.dir = Direction::Down.vector();
            *navigation_lock = true;
        }
    }

    fn handle_snake_speed(&mut self) {
        let SnakeGame {
            snake,
            fruit,
            navigation_lock,
            speed,
            game_over,
            score,
            last_update,
        } = self;

        if get_time() - *last_update > *speed {
            *last_update = get_time();
            snake.body.push_front(snake.head);
            snake.head = (snake.dir.0 + snake.head.0, snake.dir.1 + snake.head.1);

            if snake.head == *fruit {
                *fruit = (rand::gen_range(0, SQUARES), rand::gen_range(0, SQUARES));
                *score += 100;
                *speed *= 0.99;
            } else {
                snake.body.pop_back();
            }

            if snake.head.0 < 0
                || snake.head.1 < 0
                || snake.head.0 >= SQUARES
                || snake.head.1 >= SQUARES
            {
                *game_over = true;
            }

            for (x, y) in &snake.body {
                if *x == snake.head.0 && *y == snake.head.1 {
                    *game_over = true;
                }
            }

            *navigation_lock = false;
        }
    }

    fn draw_game(&self) {
        clear_background(LIGHTGRAY);

        let game_size = screen_width().min(screen_height());
        let offset_x = (screen_width() - game_size) / 2. + 10.;
        let offset_y = (screen_height() - game_size) / 2. + 10.;
        let sq_size = (screen_height() - offset_y * 2.) / SQUARES as f32;

        draw_rectangle(offset_x, offset_y, game_size - 20., game_size - 20., WHITE);

        for i in 1..SQUARES {
            draw_line(
                offset_x,
                offset_y + sq_size * i as f32,
                screen_width() - offset_x,
                offset_y + sq_size * i as f32,
                2.,
                LIGHTGRAY,
            );
        }

        for i in 1..SQUARES {
            draw_line(
                offset_x + sq_size * i as f32,
                offset_y,
                offset_x + sq_size * i as f32,
                screen_height() - offset_y,
                2.,
                LIGHTGRAY,
            );
        }

        draw_rectangle(
            offset_x + self.snake.head.0 as f32 * sq_size,
            offset_y + self.snake.head.1 as f32 * sq_size,
            sq_size,
            sq_size,
            DARKGREEN,
        );

        for (x, y) in &self.snake.body {
            draw_rectangle(
                offset_x + *x as f32 * sq_size,
                offset_y + *y as f32 * sq_size,
                sq_size,
                sq_size,
                LIME,
            );
        }

        draw_rectangle(
            offset_x + self.fruit.0 as f32 * sq_size,
            offset_y + self.fruit.1 as f32 * sq_size,
            sq_size,
            sq_size,
            GOLD,
        );

        draw_text(
            format!("Score {}", self.score).as_str(),
            20.,
            20.,
            20.0,
            DARKGRAY,
        );
    }

    fn handle_loss(&mut self) {
        clear_background(WHITE);

        let text = "Game Over. Press [enter] to play again.";
        let font_size = 30.;
        let text_size = measure_text(text, None, font_size as _, 1.0);

        draw_text(
            text,
            screen_width() / 2. - text_size.width / 2.,
            screen_height() / 2. + text_size.height / 2.,
            font_size,
            DARKGRAY,
        );

        if is_key_down(KeyCode::Enter) {
            self.snake.head = (0, 0);
            self.snake.dir = (1, 0);
            self.snake.body = LinkedList::new();
            self.fruit = (rand::gen_range(0, SQUARES), rand::gen_range(0, SQUARES));
            self.score = 0;
            self.speed = 0.3;
            self.last_update = get_time();
            self.game_over = false;
        }
    }

    async fn start_game(&mut self) {
        loop {
            if !self.game_over {
                self.handle_direction();
                self.handle_snake_speed();

                if !self.game_over {
                    self.draw_game();
                }
            } else {
                self.handle_loss();
            }
            next_frame().await
        }
    }
}

#[macroquad::main("Snake")]
async fn main() {
    let mut snake_game = SnakeGame::default();

    snake_game.start_game().await;
}

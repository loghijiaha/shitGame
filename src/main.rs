extern crate sdl2;

use std::collections::HashSet;
use std::path::Path;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use std::time::Duration;
use sdl2::render::TextureQuery;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const BALL_RADIUS: i32 = 20;
const BALL_SPEED: i32 = 5;
const RECT_SPEED: i32 = 5;
const MOVEMENT_INCREMENT: i32 = 50;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Bouncing Ball", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut ball_pos = Point::new(WINDOW_WIDTH as i32 / 2, WINDOW_HEIGHT as i32 / 2);
    let mut ball_velocity = Point::new(BALL_SPEED, BALL_SPEED);
    let mut score = 0;

    let mut player_x = 0;
    let mut player_y = 0;

    let mut red_rect = Rect::new(player_x, player_y, 50, 50);

    let mut desired_pos =  red_rect;

    let mut keys_pressed: HashSet<Keycode> = HashSet::new();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(key),
                    ..
                } => {
                    keys_pressed.insert(key);
                }
                Event::KeyUp {
                    keycode: Some(key),
                    ..
                } => {
                    keys_pressed.remove(&key);
                }
                _ => {}
            }
        }

        let mut desired_pos = red_rect;

        if keys_pressed.contains(&Keycode::Up) | keys_pressed.contains(&Keycode::W) {
            desired_pos.set_y(desired_pos.y() - MOVEMENT_INCREMENT);
        }
        if keys_pressed.contains(&Keycode::Down) | keys_pressed.contains(&Keycode::S) {
            desired_pos.set_y(desired_pos.y() + MOVEMENT_INCREMENT);
        }
        if keys_pressed.contains(&Keycode::Left) | keys_pressed.contains(&Keycode::A) {
            desired_pos.set_x(desired_pos.x() - MOVEMENT_INCREMENT);
        }
        if keys_pressed.contains(&Keycode::Right) | keys_pressed.contains(&Keycode::D) {
            desired_pos.set_x(desired_pos.x() + MOVEMENT_INCREMENT);
        }

        if (keys_pressed.contains(&Keycode::Up) | keys_pressed.contains(&Keycode::W)) && (keys_pressed.contains(&Keycode::Left) | keys_pressed.contains(&Keycode::A)){
            desired_pos.set_x(desired_pos.x() - MOVEMENT_INCREMENT);
            desired_pos.set_y(desired_pos.y() - MOVEMENT_INCREMENT);
        }
        if (keys_pressed.contains(&Keycode::Up) | keys_pressed.contains(&Keycode::W)) &&  (keys_pressed.contains(&Keycode::Right) | keys_pressed.contains(&Keycode::D)) {
            desired_pos.set_x(desired_pos.x() + MOVEMENT_INCREMENT);
            desired_pos.set_y(desired_pos.y() - MOVEMENT_INCREMENT);
        }
        if (keys_pressed.contains(&Keycode::Down) | keys_pressed.contains(&Keycode::S)) && (keys_pressed.contains(&Keycode::Left) | keys_pressed.contains(&Keycode::A)) {
            desired_pos.set_x(desired_pos.x() - MOVEMENT_INCREMENT);
            desired_pos.set_y(desired_pos.y() + MOVEMENT_INCREMENT);
        }
        if (keys_pressed.contains(&Keycode::Down) | keys_pressed.contains(&Keycode::S)) && (keys_pressed.contains(&Keycode::Right) | keys_pressed.contains(&Keycode::D)) {
            desired_pos.set_x(desired_pos.x() + MOVEMENT_INCREMENT);
            desired_pos.set_y(desired_pos.y() + MOVEMENT_INCREMENT);
        }

        if red_rect != desired_pos {
            let delta_x = desired_pos.x() - red_rect.x();
            let delta_y = desired_pos.y() - red_rect.y();

            let direction_x = delta_x.signum();
            let direction_y = delta_y.signum();

            let new_pos_x = red_rect.x() + direction_x * RECT_SPEED;
            let new_pos_y = red_rect.y() + direction_y * RECT_SPEED;

            red_rect.set_x(new_pos_x);
            red_rect.set_y(new_pos_y);
        }

        ball_pos += ball_velocity;

        // Check collision with red rectangle
        if ball_pos.x >= red_rect.x() && ball_pos.x <= red_rect.x() + red_rect.width() as i32
            && ball_pos.y >= red_rect.y() && ball_pos.y <= red_rect.y() + red_rect.height() as i32
        {
            score += 1;
        }

        // Bounce off the walls
        if ball_pos.x < BALL_RADIUS || ball_pos.x > WINDOW_WIDTH as i32 - BALL_RADIUS {
            ball_velocity = Point::new(-ball_velocity.x, ball_velocity.y);
        }
        if ball_pos.y < BALL_RADIUS || ball_pos.y > WINDOW_HEIGHT as i32 - BALL_RADIUS {
            ball_velocity = Point::new(ball_velocity.x, -ball_velocity.y);
        }

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        canvas.set_draw_color(Color::RED);
        canvas.draw_rect(red_rect).unwrap();
        canvas.fill_rect(red_rect).unwrap();

        canvas.set_draw_color(Color::BLUE);
        draw_circle(&mut canvas, ball_pos, BALL_RADIUS);

        canvas.present();
        println!("Score: {}", score);

        if score == 100 {
            break 'running;
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn draw_circle(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, center: Point, radius: i32) {
    let mut x = radius - 1;
    let mut y = 0;
    let mut dx = 1;
    let mut dy = 1;
    let mut err = dx - (radius << 1);

    while x >= y {
        canvas.draw_point(center + Point::new(x, y)).unwrap();
        canvas.draw_point(center + Point::new(-x, y)).unwrap();
        canvas.draw_point(center + Point::new(x, -y)).unwrap();
        canvas.draw_point(center + Point::new(-x, -y)).unwrap();
        canvas.draw_point(center + Point::new(y, x)).unwrap();
        canvas.draw_point(center + Point::new(-y, x)).unwrap();
        canvas.draw_point(center + Point::new(y, -x)).unwrap();
        canvas.draw_point(center + Point::new(-y, -x)).unwrap();

        if err <= 0 {
            y += 1;
            err += dy;
            dy += 2;
        }
        if err > 0 {
            x -= 1;
            dx += 2;
            err += dx - (radius << 1);
        }
    }
}
// render a text in the canvas at the given position
fn render_win() {


}

fn show_alert_window(sdl_context: &sdl2::Sdl) {
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Swallowed your salary", 400, 200)
        .position_centered()
        .build()
        .unwrap();

    let mut alert_canvas = window.into_canvas().build().unwrap();

    alert_canvas.set_draw_color(Color::RED);
    alert_canvas.clear();
    alert_canvas.present();

    ::std::thread::sleep(Duration::new(2, 0));
}


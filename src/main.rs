#![allow(unused)]

use std::array::IntoIter;

use background::Background;
use macroquad::audio::{load_sound_from_bytes, play_sound_once};
use macroquad::experimental::coroutines::wait_seconds;
use macroquad::miniquad::window::screen_size;
use macroquad::prelude::*;
use macroquad::{experimental::scene::clear, rand::srand};

use bird::{Bird, BirdResource};
use score::{Score, ScoreTexture};

mod background;
mod bird;
mod pipe;
mod score;
mod utils;

pub mod consts {
    pub const GRAVITY: f32 = 10.0 * 100.0;
}

#[derive(Debug, Clone, Copy)]
enum GameState {
    Game,
    GameOver,
    Menu,
}

fn window_conf() -> Conf {
    Conf {
        window_height: 800,
        window_width: 800 * 9 / 16,
        window_title: "Flappy Bird".to_string(),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    srand(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    );

    let mut game_state = GameState::Menu;

    let bird_texture = BirdResource {
        upflap: &texture!("yellowbird-upflap.png"),
        midflap: &texture!("yellowbird-midflap.png"),
        downflap: &texture!("yellowbird-downflap.png"),
        wing_sound: &sound!("wing.ogg"),
        swoosh_sound: &sound!("swoosh.ogg"),
    };

    let point = &sound!("point.ogg");
    let hit = &sound!("hit.ogg");
    let die = &sound!("die.ogg");
    let base_texture = &texture!("base.png");
    let background_texture = &texture!("background-day.png");

    let menu_texture = &texture!("message.png");
    let game_over_texture = &texture!("gameover.png");

    let score_texture = ScoreTexture {
        digit_0: &texture!("0.png"),
        digit_1: &texture!("1.png"),
        digit_2: &texture!("2.png"),
        digit_3: &texture!("3.png"),
        digit_4: &texture!("4.png"),
        digit_5: &texture!("5.png"),
        digit_6: &texture!("6.png"),
        digit_7: &texture!("7.png"),
        digit_8: &texture!("8.png"),
        digit_9: &texture!("9.png"),
    };

    let pipe = &texture!("pipe-green.png");
    let scr_height = screen_height() - base_texture.height();

    let initial_bird = Bird::new(
        bird_texture,
        vec2(screen_width() * 0.1, screen_height() * 0.2),
        screen_height() * 0.1,
        300.0,
        scr_height,
    );

    let mut bird = initial_bird;
    let mut scroll_speed = 100.0;
    let mut background = Background::new(background_texture, base_texture, scroll_speed);
    let mut pipes: Vec<pipe::Pipe> = vec![];
    let mut clock = 5.0;
    let mut pressed = false;
    let mut score = Score::new(
        score_texture,
        scr_height * 0.1,
        0.0,
        initial_bird.get_bounding_rect().h * 0.5,
    );

    loop {
        match game_state {
            GameState::Game => {
                clock += get_frame_time() * !bird.is_touching_ground() as u8 as f32;
                if clock > 5.0 {
                    scroll_speed += score.get_score() as f32 * 0.5;
                    background.set_speed(scroll_speed);

                    pipes.push(pipe::Pipe::new(
                        pipe,
                        scr_height * 0.8,
                        initial_bird.get_bounding_rect().h * 3.0,
                        scroll_speed,
                        scr_height,
                    ));
                    clock = 0.0;
                }

                if is_mouse_button_pressed(MouseButton::Left) && !pressed {
                    pressed = true;
                    bird.flap();
                } else {
                    pressed = false;
                }

                bird.update();
                if !bird.is_touching_ground() {
                    background.update();
                    pipes.iter_mut().for_each(|p| {
                        p.update();

                        let bound_rects = p.get_bounding_rects();
                        if bird.is_colliding(bound_rects[0]) || bird.is_colliding(bound_rects[1]) {
                            play_sound_once(hit);
                            play_sound_once(die);
                            game_state = GameState::GameOver;
                        }
                    });
                }

                pipes.retain(|p| {
                    if !p.is_invisible() {
                        true
                    } else {
                        score.increment_score();
                        play_sound_once(point);
                        false
                    }
                });
            }

            GameState::GameOver if is_mouse_button_down(MouseButton::Left) => {
                pipes.clear();
                clock = 3.0;
                score.set_score(0);
                bird = initial_bird;

                game_state = GameState::Menu;
            }

            GameState::Menu if is_mouse_button_pressed(MouseButton::Left) => {
                game_state = GameState::Game;
            }

            _ => {}
        }

        clear();

        background.draw_bg();

        if let GameState::Menu = game_state {
            draw_texture_ex(
                &menu_texture,
                screen_width() * 0.2,
                scr_height * 0.2,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(screen_width() * 0.6, scr_height * 0.6)),
                    ..Default::default()
                },
            );
        } else {
            pipes.iter_mut().for_each(|p| p.draw());

            if !matches!(game_state, GameState::GameOver) {
                bird.draw()
            } else {
                draw_texture(
                    &game_over_texture,
                    screen_width() * 0.5 - game_over_texture.size().x * 0.5,
                    scr_height * 0.5,
                    WHITE,
                );
            }

            score.draw();
        }

        background.draw_base();

        next_frame().await;
    }
}

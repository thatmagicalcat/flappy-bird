use macroquad::{
    audio::{play_sound_once, Sound},
    prelude::*,
};

use super::consts::GRAVITY;

#[derive(Debug, Clone, Copy)]
pub struct BirdResource<'a> {
    pub upflap: &'a Texture2D,
    pub midflap: &'a Texture2D,
    pub downflap: &'a Texture2D,
    pub wing_sound: &'a Sound,
    pub swoosh_sound: &'a Sound,
}

#[derive(Debug, Clone, Copy)]
pub struct Bird<'a> {
    bounding_rect: Rect,
    velocity_y: f32,
    jump_velocity: f32,
    screen_height: f32,

    resource: BirdResource<'a>,
    touching_ground: bool,
}

impl<'a> Bird<'a> {
    pub fn new(
        resource: BirdResource<'a>,
        position: Vec2,
        height: f32,
        jump_velocity: f32,
        screen_height: f32,
    ) -> Self {
        let aspect_ratio = resource.upflap.size().x / resource.upflap.size().y;

        Self {
            jump_velocity,
            screen_height,
            resource,

            touching_ground: false,
            velocity_y: 0.0,
            bounding_rect: Rect {
                x: position.x,
                y: position.y,
                w: height * aspect_ratio,
                h: height,
            },
        }
    }

    pub fn flap(&mut self) {
        if self.velocity_y >= 500.0 {
            play_sound_once(self.resource.swoosh_sound);
        } else {
            play_sound_once(self.resource.wing_sound);
        }

        self.velocity_y = -self.jump_velocity;
    }

    pub fn is_touching_ground(&self) -> bool {
        self.touching_ground
    }

    pub fn update(&mut self) {
        self.velocity_y += GRAVITY * get_frame_time();
        self.bounding_rect.y += self.velocity_y * get_frame_time();
            self.touching_ground = false;

        if self.bounding_rect.y + self.bounding_rect.h > self.screen_height {
            self.touching_ground = true;
            self.bounding_rect.y = self.screen_height - self.bounding_rect.h;
            self.velocity_y = 0.0;
        }

        if self.bounding_rect.y < 0.0 {
            self.bounding_rect.y = 0.0;
            self.velocity_y = 0.0;
        }
    }

    pub fn draw(&self) {
        let Rect { x, y, w, h } = self.bounding_rect;

        let (text, rotation) = if self.velocity_y > 100.0 {
            (self.resource.upflap, 10f32.to_radians())
        } else if self.velocity_y < 100.0 && self.velocity_y > -100.0 {
            (self.resource.midflap, 0.0)
        } else {
            (self.resource.downflap, -10f32.to_radians())
        };

        draw_texture_ex(
            text,
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(w, h)),
                rotation,
                ..Default::default()
            },
        );
    }

    pub fn is_colliding(&self, other: Rect) -> bool {
        self.bounding_rect.intersect(other).is_some()
    }

    pub fn get_bounding_rect(&self) -> Rect {
        self.bounding_rect
    }
}

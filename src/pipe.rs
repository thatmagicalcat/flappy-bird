use macroquad::{prelude::*, rand::gen_range};

#[derive(Debug, Clone, Copy)]
pub struct Pipe<'a> {
    upper_pipe: Rect,
    lower_pipe: Rect,
    speed: f32,
    screen_height: f32,

    texture: &'a Texture2D,
}

impl<'a> Pipe<'a> {
    pub fn new(texture: &'a Texture2D, height: f32, gap: f32, speed: f32, screen_height: f32) -> Self {
        let aspect_ratio = texture.size().x / texture.size().y;

        let width = height * aspect_ratio;

        let pos_x = screen_width() + width;
        let pos_y_down = gen_range(screen_height * 0.2, screen_height * 0.8);
        let pos_y_up = pos_y_down - gap - height;

        Self {
            screen_height,
            speed,
            texture,

            upper_pipe: Rect {
                x: pos_x,
                y: pos_y_up,
                w: width,
                h: height,
            },

            lower_pipe: Rect {
                x: pos_x,
                y: pos_y_down,
                w: width,
                h: height,
            },
        }
    }

    pub fn get_bounding_rects(&self) -> [Rect; 2] {
        [self.upper_pipe, self.lower_pipe]
    }

    pub fn draw(&self) {
        draw_texture_ex(
            &self.texture,
            self.upper_pipe.x,
            self.upper_pipe.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(self.upper_pipe.size()),
                rotation: std::f32::consts::PI,
                ..Default::default()
            },
        );

        draw_texture_ex(
            &self.texture,
            self.lower_pipe.x,
            self.lower_pipe.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(self.lower_pipe.size()),
                ..Default::default()
            },
        );
    }

    pub fn update(&mut self) {
        self.upper_pipe.x -= self.speed * get_frame_time();
        self.lower_pipe.x -= self.speed * get_frame_time();
    }

    pub fn is_invisible(&self) -> bool {
        self.upper_pipe.x + self.upper_pipe.w < 0.0
    }
}

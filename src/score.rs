use macroquad::prelude::*;

pub struct ScoreTexture<'a> {
    pub digit_0: &'a Texture2D,
    pub digit_1: &'a Texture2D,
    pub digit_2: &'a Texture2D,
    pub digit_3: &'a Texture2D,
    pub digit_4: &'a Texture2D,
    pub digit_5: &'a Texture2D,
    pub digit_6: &'a Texture2D,
    pub digit_7: &'a Texture2D,
    pub digit_8: &'a Texture2D,
    pub digit_9: &'a Texture2D,
}

pub struct Score<'a> {
    texture: ScoreTexture<'a>,
    digit_width: f32,
    pos_y: f32,
    gap: f32,
    score: usize,
    height: f32,
    width: f32,
}

impl<'a> Score<'a> {
    pub fn new(texture: ScoreTexture<'a>, starting_y_pos: f32, gap: f32, height: f32) -> Self {
        let aspect_ratio = texture.digit_0.width() / texture.digit_0.height();

        Self {
            score: 0,
            digit_width: texture.digit_0.size().x,
            pos_y: starting_y_pos,
            width: height * aspect_ratio,

            gap,
            height,
            texture,
        }
    }

    pub fn increment_score(&mut self) {
        self.score += 1;
    }

    pub fn decrement_score(&mut self) {
        self.score -= 1;
    }

    pub fn set_score(&mut self, new_score: usize) {
        self.score = new_score;
    }

    pub fn get_score(&self) -> usize {
        self.score
    }

    pub fn draw(&self) {
        let score = self.score.to_string();
        let num_digits = score.len() as f32;
        let center_pos_x = screen_width() * 0.5;
        let total_width = num_digits * self.digit_width + self.gap * (num_digits - 1.0);
        let starting_pos_x = center_pos_x - total_width * 0.5;

        for (idx, digit) in score.chars().enumerate() {
            let pos_x = starting_pos_x
                + self.digit_width * idx as f32
                + self.gap * (idx as f32 - 1.0) as f32;

            draw_texture_ex(
                self.get_digit_texture(digit as u8 - b'0'),
                pos_x,
                self.pos_y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(self.width, self.height)),
                    ..Default::default()
                },
            );
        }
    }

    fn get_digit_texture(&self, idx: u8) -> &Texture2D {
        match idx {
            0 => self.texture.digit_0,
            1 => self.texture.digit_1,
            2 => self.texture.digit_2,
            3 => self.texture.digit_3,
            4 => self.texture.digit_4,
            5 => self.texture.digit_5,
            6 => self.texture.digit_6,
            7 => self.texture.digit_7,
            8 => self.texture.digit_8,
            9 => self.texture.digit_9,

            _ => panic!(),
        }
    }
}

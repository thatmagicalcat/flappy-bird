use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct Background<'a> {
    texture_main: &'a Texture2D,
    texture_base: &'a Texture2D,
    speed: f32,

    backgrounds: Vec<Rect>,
    bases: Vec<Rect>,
}

impl<'a> Background<'a> {
    pub fn new(texture_main: &'a Texture2D, texture_base: &'a Texture2D, speed: f32) -> Self {
        let h = texture_base.height();
        let y = screen_height() - h;

        Self {
            backgrounds: vec![
                Rect::new(screen_width() * 0.0, 0.0, screen_width(), screen_height()),
                Rect::new(screen_width() * 1.0, 0.0, screen_width(), screen_height()),
                Rect::new(screen_width() * 2.0, 0.0, screen_width(), screen_height()),
            ],

            bases: vec![
                Rect::new(screen_width() * 0.0, y, screen_width(), h),
                Rect::new(screen_width() * 1.0, y, screen_width(), h),
                Rect::new(screen_width() * 2.0, y, screen_width(), h),
            ],

            texture_main,
            texture_base,
            speed,
        }
    }

    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed;
    }

    pub fn draw_bg(&self) {
        for bg in &self.backgrounds {
            let &Rect { x, y, w, h } = bg;

            draw_texture_ex(
                &self.texture_main,
                x,
                y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(w, h)),
                    ..Default::default()
                },
            );
        }
    }

    pub fn draw_base(&self) {
        for base in &self.bases {
            let &Rect { x, y, w, h } = base;

            draw_texture_ex(
                &self.texture_base,
                x,
                y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(w, h)),
                    ..Default::default()
                },
            );
        }
    }

    pub fn update(&mut self) {
        for bg in &mut self.backgrounds {
            bg.x -= self.speed * get_frame_time() * 0.5;
        }

        for base in &mut self.bases {
            base.x -= self.speed * get_frame_time();
        }

        let pre_len = self.backgrounds.len();
        self.backgrounds.retain(|&Rect { x, w, .. }| x + w > 0.0);

        if self.backgrounds.len() != pre_len {
            let (x, y) = self.backgrounds.last().unwrap().point().into();
            self.backgrounds.push(Rect::new(
                x + screen_width(),
                y,
                screen_width(),
                screen_height(),
            ));
        }

        let pre_len = self.bases.len();
        self.bases.retain(|&Rect { x, w, .. }| x + w > 0.0);

        if self.bases.len() != pre_len {
            let &Rect { x, y, w, h } = self.bases.last().unwrap();
            self.bases.push(Rect::new(x + screen_width(), y, w, h));
        }
    }
}

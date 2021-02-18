use std::time::Duration;

use ggez::GameResult;

use crate::rendering::SpriteSheet;

pub struct SpriteSheetAnimation {
    fps: f32,
    spritesheet: SpriteSheet,
    frame_duration: f32,
    current_frame_time: f32,
}

impl SpriteSheetAnimation {
    pub fn new(spritesheet: SpriteSheet, fps: f32) -> Self {
        SpriteSheetAnimation {
            spritesheet,
            fps,
            frame_duration: 1.0 / fps,
            current_frame_time: 0.0,
        }
    }

    pub fn update(&mut self, deltatime: Duration) {
        self.current_frame_time += deltatime.as_secs_f32();
        if self.current_frame_time > self.frame_duration {
            self.current_frame_time = 0.0;
            self.spritesheet.set_next();
        }
    }

    pub fn reset(&mut self) {
        self.current_frame_time = 0.0;
        self.spritesheet.set_active(0);
    }

    pub fn set_fps(&mut self, fps: f32) {
        self.fps = fps;
        self.frame_duration = 1.0 / fps;
    }

    pub fn fps(&self) -> f32 {
        self.fps
    }

    pub fn get_drawable(&self) -> &SpriteSheet {
        &self.spritesheet
    }
}

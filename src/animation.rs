use std::time::Duration;

use ggez::graphics::Drawable;

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

struct Rule<T> {
    to: usize,
    trigger: Box<dyn Fn(T) -> bool>,
}
pub struct StateMachine<T> {
    rules: Vec<Vec<Rule<T>>>,
    current_state: usize,
}

impl<T: Clone> StateMachine<T> {
    pub fn new(state_count: usize) -> Self {
        Self {
            current_state: 0,
            rules: (0..state_count).into_iter().map(|_| Vec::new()).collect(),
        }
    }

    pub fn add_rule<F>(&mut self, from: usize, to: usize, callback: F)
    where
        F: Fn(T) -> bool + 'static,
    {
        self.rules[from].push(Rule {
            to,
            trigger: Box::new(callback),
        });
    }

    pub fn update_once(&mut self, monitor: T) -> bool {
        for rule in self.rules[self.current_state].iter() {
            if (rule.trigger)(monitor.clone()) {
                self.current_state = rule.to;
                return true;
            }
        }
        false
    }

    pub fn update_full(&mut self, monitor: T) -> bool {
        let mut updated = false;
        while self.update_once(monitor.clone()) {
            updated = true;
        }
        updated
    }

    pub fn update_with_limit(&mut self, monitor: T, limit: u32) -> bool {
        for _ in 0..limit {
            if self.update_once(monitor.clone()) {
                return true;
            }
        }
        false
    }

    pub fn state(&self) -> usize {
        self.current_state
    }

    pub fn set_state(&mut self, state: usize) {
        self.current_state = state;
    }
}

pub struct SpriteAnimator<T> {
    animations: Vec<SpriteSheetAnimation>,
    pub state_machine: StateMachine<T>
}

impl<T: Clone> SpriteAnimator<T> {
    pub fn from_animations(animations: Vec<SpriteSheetAnimation>) -> Self {
        Self {
            state_machine: StateMachine::<T>::new(animations.len()),
            animations
        }
    }

    pub fn get_animation(&self) -> &SpriteSheetAnimation {
        &self.animations[self.state_machine.current_state]
    }

    pub fn get_animation_mut(&mut self) -> &mut SpriteSheetAnimation {
        &mut self.animations[self.state_machine.current_state]
    }
}
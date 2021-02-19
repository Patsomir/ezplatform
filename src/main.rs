use ezplatform::game::{EzPlatform, SCREEN_HEIGHT, SCREEN_WIDTH};
use ggez::{ContextBuilder, GameResult, event};

fn main() -> GameResult {
    let (mut ctx, mut event_loop) = ContextBuilder::new("EzPlatform", "Plamen Nikolov")
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .build()
        .unwrap();

    let mut ez_platform = EzPlatform::new(&mut ctx);

    event::run(&mut ctx, &mut event_loop, &mut ez_platform)
}
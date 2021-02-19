use ggez::{
    event::{self, EventHandler},
    graphics::FilterMode,
    mint::Point2,
};
use ggez::{
    event::{KeyCode, KeyMods},
    graphics::{Color, Rect},
};
use ggez::{graphics, input::keyboard, mint::Vector2};
use ggez::{graphics::Image, timer};
use ggez::{Context, ContextBuilder, GameResult};

use ezplatform::{collision::DynamicCollider, rendering::SpriteSheet};
use ezplatform::{animation::SpriteAnimator, rendering::TilemapRenderer, world::World};
use ezplatform::{animation::SpriteSheetAnimation, physics::PhysicsObject};
use ezplatform::{
    collision::TilemapCollider, movement::MovementController, rendering::WorldDrawable,
};

const SCREEN_WIDTH: f32 = 1200.0;
const SCREEN_HEIGHT: f32 = 600.0;
const DISTANCE: f32 = 7.0;
const RED: Color = Color::new(1.0, 0.0, 0.0, 1.0);

fn main() {
    let (mut ctx, mut event_loop) = ContextBuilder::new("EzPlatform", "Plamen Nikolov")
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .build()
        .unwrap();

    let world: World = World::new(SCREEN_WIDTH, SCREEN_HEIGHT, DISTANCE);
    let mut my_game = MyGame::new(&mut ctx, world);

    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

const MASS: f32 = 3.0;
const SIZE: f32 = 1.0;
const MOVE_FORCE: f32 = 1.2;
const JUMP_IMPULSE: f32 = 0.15;
const MAX_SPEED: f32 = 0.08;
const MOVE_SPEED_DECAY: f32 = 0.9;
const GRAVITY_ACCELERATION: f32 = 0.5;

const ZERO_POINT: Point2<f32> = Point2 { x: 0.0, y: 0.0 };

const GROUND_TEMPLATE: &[&[u32]] = &[
    &[2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    &[7, 3, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    &[7, 3, 3, 3, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 8],
    &[7, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    &[0, 0, 0, 7, 3, 3, 8, 0, 0, 0, 0, 0, 0, 0],
];

struct MyGame {
    world: World,
    total_time: std::time::Duration,

    tilemap_collider: TilemapCollider,
    controller: MovementController,
    can_jump: bool,

    orientation: i8,
    player_animator: SpriteAnimator<Vector2<f32>>,
    ground: TilemapRenderer,
}

impl MyGame {
    pub fn new(ctx: &mut Context, world: World) -> MyGame {
        let body = DynamicCollider::from_rect(Rect::new(ZERO_POINT.x, ZERO_POINT.y, SIZE, SIZE), MASS);
        let controller = MovementController::from_components(
            body,
            MOVE_FORCE,
            JUMP_IMPULSE,
            MAX_SPEED,
            MOVE_SPEED_DECAY,
            GRAVITY_ACCELERATION,
        );

        let idle_image = Image::new(ctx, "/placeholder.png").unwrap();
        let idle_sprites = SpriteSheet::new(idle_image, 1, 1, 1);
        let idle_animation = SpriteSheetAnimation::new(idle_sprites, 1.0);

        let jump_image = Image::new(ctx, "/jump.png").unwrap();
        let jump_sprites = SpriteSheet::new(jump_image, 1, 1, 1);
        let jump_animation = SpriteSheetAnimation::new(jump_sprites, 1.0);

        let fall_image = Image::new(ctx, "/fall.png").unwrap();
        let fall_sprites = SpriteSheet::new(fall_image, 1, 1, 1);
        let fall_animation = SpriteSheetAnimation::new(fall_sprites, 1.0);

        let walking_image = Image::new(ctx, "/walking.png").unwrap();
        let walking_sprites = SpriteSheet::new(walking_image, 3, 2, 6);
        let walking_animation = SpriteSheetAnimation::new(walking_sprites, 30.0);

        let mut player_animator: SpriteAnimator<Vector2<f32>> =
            SpriteAnimator::from_animations(vec![
                idle_animation,
                walking_animation,
                jump_animation,
                fall_animation,
            ]);
        player_animator.add_rule(0, 1, |velocity| velocity.x.abs() > 0.01);
        player_animator.add_rule(1, 0, |velocity| velocity.x.abs() < 0.01);
        player_animator.add_rule(0, 2, |velocity| velocity.y > 0.01);
        player_animator.add_rule(1, 2, |velocity| velocity.y > 0.01);
        player_animator.add_rule(0, 2, |velocity| velocity.y < -0.01);
        player_animator.add_rule(1, 2, |velocity| velocity.y < -0.01);
        player_animator.add_rule(2, 3, |velocity| velocity.y <= 0.0);
        player_animator.add_rule(3, 0, |velocity| velocity.y >= 0.0);

        let mut ground_image = Image::new(ctx, "/ground.png").unwrap();
        ground_image.set_filter(FilterMode::Nearest);
        let ground_sprites = SpriteSheet::new(ground_image, 4, 4, 16);
        let ground = TilemapRenderer::from_components(
            ground_sprites,
            GROUND_TEMPLATE,
            1.0,
            1.0,
            Point2 { x: 5, y: 5 },
        );

        let tilemap_collider = TilemapCollider::from(&ground);

        MyGame {
            world,
            total_time: std::time::Duration::new(0, 0),
            controller,
            can_jump: false,
            tilemap_collider,
            player_animator,
            orientation: 1,
            ground,
        }
    }
}

impl EventHandler for MyGame {
    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        if self.can_jump && keycode == KeyCode::W {
            self.controller.jump();
            self.can_jump = false;
        }
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let deltatime = timer::delta(ctx);
        self.total_time += deltatime;
        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::A) {
            self.controller.move_left();
            self.orientation = -1;
        } else if keyboard::is_key_pressed(ctx, keyboard::KeyCode::D) {
            self.controller.move_right();
            self.orientation = 1;
        } else {
            self.controller.stop();
        }
        self.controller.update(deltatime);

        let player_rect = Rect::new(
            self.controller.body.position().x,
            self.controller.body.position().y,
            SIZE,
            SIZE,
        );
        let collisions = self.tilemap_collider.get_collisions(player_rect);
        // if collisions.len() != 0 && self.controller.body.velocity().y < 0.0 {
        //     let max_rect = collisions.iter().fold(collisions[0], |a, b| {
        //         if a.y < b.y {
        //             return *b;
        //         }
        //         a
        //     });
        //     if self.controller.body.position().y > max_rect.y {
        //         self.controller.body.position_mut().y = max_rect.y + max_rect.h / 2.0 + SIZE / 2.0;
        //         self.controller.body.velocity_mut().y = 0.0;
        //         self.can_jump = true;
        //     }
        // }
        self.controller.body.resolve_collisions(&collisions);
        self.can_jump = true;

        // if self.controller.body.position.x < -DISTANCE * 1.5 {
        //     self.controller.body.velocity.x = 0.0;
        //     self.controller.body.position.x = -DISTANCE * 1.5;
        // }
        // if self.controller.body.position.x > DISTANCE * 1.5 {
        //     self.controller.body.velocity.x = 0.0;
        //     self.controller.body.position.x = DISTANCE * 1.5;
        // }

        // let camera_pos = self.world.camera_position();
        // self.world.look_at(Point2 { x: camera_pos.x + (self.controller.body.position.x - camera_pos.x) * deltatime.as_secs_f32() * 10.0, y: camera_pos.y });

        if self.controller.body.position().y < -DISTANCE - 2.0 {
            self.controller.body.position_mut().y = DISTANCE + 2.0;
        }
        if self.controller.body.position().x < -DISTANCE * 2.0 - 2.0 {
            self.controller.body.position_mut().x = DISTANCE * 2.0 + 2.0;
        }
        if self.controller.body.position().x > DISTANCE * 2.0 + 2.0 {
            self.controller.body.position_mut().x = -DISTANCE * 2.0 - 2.0;
        }

        self.player_animator
            .update(self.controller.body.velocity(), deltatime);

        // let time_seconds = self.total_time.as_secs_f32();
        // let speed = 4.0;
        // let radius = 5.0;

        // self.world.look_at(Point2 { x: radius * (speed * time_seconds).sin(), y: radius *  (speed * time_seconds).cos() });
        // self.world.set_distance(DISTANCE + 10.0 * (speed * time_seconds).sin());

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::new(0.1, 0.08, 0.05, 1.0));

        // draw_rectangle(ctx, self.world.new_screen_param(0.0, -1.0, 2.0, 1.0));
        // draw_rectangle(ctx, self.world.new_screen_param(1.0, 0.0, 1.0, 2.0));
        // draw_rectangle(ctx, self.world.new_screen_param(0.0, 1.0, 2.0, 1.0));
        // draw_rectangle(ctx, self.world.new_screen_param(-1.0, 0.0, 1.0, 2.0));
        // draw_rectangle(ctx, self.world.new_screen_param(0.0, 0.0, 1.5, 1.5).color(RED).rotation(-5.0 * self.total_time.as_secs_f32()));

        //draw_rect_in_world(ctx, Rect::new(self.controller.body.position.x, self.controller.body.position.y, SIZE, SIZE), graphics::WHITE, &self.world);
        //self.tilemap_collider.draw_in_world(ctx, RED, &self.world);
        self.ground
            .draw_in_world(ctx, &self.world, Rect::default())
            .unwrap();

        self.player_animator
            .get_drawable()
            .draw_in_world(
                ctx,
                &self.world,
                Rect::new(
                    self.controller.body.position().x,
                    self.controller.body.position().y,
                    self.orientation as f32 * SIZE,
                    SIZE,
                ),
            )
            .unwrap();

        graphics::present(ctx).expect("Failed to present");
        Ok(())
    }
}

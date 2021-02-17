use ggez::timer;
use ggez::{
    event::{self, EventHandler},
    mint::Point2,
};
use ggez::{
    event::{KeyCode, KeyMods},
    graphics::{Color, Rect},
};
use ggez::{graphics, input::keyboard, mint::Vector2};
use ggez::{Context, ContextBuilder, GameResult};
use graphics::{DrawParam, Mesh};

use ezplatform::{collision::TilemapCollider, movement::MovementController};
use ezplatform::physics::PhysicsObject;
use ezplatform::world::World;
use ezplatform::debug::draw_rect_in_world;

const SCREEN_WIDTH: f32 = 1200.0;
const SCREEN_HEIGHT: f32 = 600.0;
const DISTANCE: f32 = 15.0;
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

const MASS: f32 = 5.0;
const SIZE: f32 = 2.0;
const MOVE_FORCE: f32 = 8.0;
const JUMP_IMPULSE: f32 = 0.5;
const MAX_SPEED: f32 = 0.5;
const MOVE_SPEED_DECAY: f32 = 6.0;
const GRAVITY_ACCELERATION: f32 = 2.0;

const ZERO_POINT: Point2<f32> = Point2 { x: 0.0, y: 0.0 };

const O: bool = true;
const X: bool = false;
const COLLIDER_TEMPLATE: &[&[bool]] = &[
    &[X, X, X, O, X, X, X, X, X, X],
    &[X, X, X, X, X, X, X, X, X, X],
    &[X, X, X, X, X, X, X, X, X, X],
    &[X, X, O, O, O, X, X, X, X, X],
    &[X, X, X, X, X, X, X, X, X, X],
    &[X, X, X, X, X, X, X, X, X, X],
    &[X, O, O, O, O, O, X, X, X, X],
    &[X, X, X, X, X, X, X, X, X, X],
    &[X, X, X, X, X, X, X, X, X, X],
    &[X, X, X, X, X, X, X, X, O, O],
    &[O, O, X, X, X, X, X, X, X, X],
    &[X, X, X, O, O, O, O, X, X, X],
];

struct MyGame {
    world: World,
    total_time: std::time::Duration,

    tilemap_collider: TilemapCollider,
    controller: MovementController,
    can_jump: bool,
}

impl MyGame {
    pub fn new(ctx: &mut Context, world: World) -> MyGame {
        let body = PhysicsObject::new(ZERO_POINT, MASS);
        let controller = MovementController::from_components(
            body,
            MOVE_FORCE,
            JUMP_IMPULSE,
            MAX_SPEED,
            MOVE_SPEED_DECAY,
            GRAVITY_ACCELERATION,
        );
        let tilemap_collider = TilemapCollider::from_components(
            COLLIDER_TEMPLATE,
            5.0,
            1.5,
            Point2 { x: 5, y: 5 }
        );

        MyGame {
            world,
            total_time: std::time::Duration::new(0, 0),
            controller,
            can_jump: false,
            tilemap_collider,
        }
    }
}

impl EventHandler for MyGame {
    fn key_down_event(
        &mut self,
        ctx: &mut Context,
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
        } else if keyboard::is_key_pressed(ctx, keyboard::KeyCode::D) {
            self.controller.move_right();
        } else {
            self.controller.stop();
        }
        self.controller.update(deltatime);

        let player_rect = Rect::new(self.controller.body.position.x, self.controller.body.position.y, SIZE, SIZE);
        let collisions = self.tilemap_collider.get_collisions(player_rect);
        if collisions.len() != 0 && self.controller.body.velocity.y < 0.0 {
            let max_rect = collisions.iter().fold(collisions[0], |a, b| {
                if a.y < b.y {
                    return *b;
                }
                a
            });
            if self.controller.body.position.y > max_rect.y {
                self.controller.body.position.y = max_rect.y + max_rect.h / 2.0 + SIZE / 2.0;
                self.controller.body.velocity.y = 0.0;
                self.can_jump = true;
            }
        }
        

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

        if self.controller.body.position.y < -DISTANCE - 2.0 {
            self.controller.body.position.y = DISTANCE + 2.0;
        }
        if self.controller.body.position.x < -DISTANCE * 2.0 - 2.0 {
            self.controller.body.position.x = DISTANCE * 2.0 + 2.0;
        }
        if self.controller.body.position.x > DISTANCE * 2.0 + 2.0 {
            self.controller.body.position.x = -DISTANCE * 2.0 - 2.0;
        }

        // let time_seconds = self.total_time.as_secs_f32();
        // let speed = 4.0;
        // let radius = 5.0;

        // self.world.look_at(Point2 { x: radius * (speed * time_seconds).sin(), y: radius *  (speed * time_seconds).cos() });
        // self.world.set_distance(DISTANCE + 10.0 * (speed * time_seconds).sin());

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        // draw_rectangle(ctx, self.world.new_screen_param(0.0, -1.0, 2.0, 1.0));
        // draw_rectangle(ctx, self.world.new_screen_param(1.0, 0.0, 1.0, 2.0));
        // draw_rectangle(ctx, self.world.new_screen_param(0.0, 1.0, 2.0, 1.0));
        // draw_rectangle(ctx, self.world.new_screen_param(-1.0, 0.0, 1.0, 2.0));
        // draw_rectangle(ctx, self.world.new_screen_param(0.0, 0.0, 1.5, 1.5).color(RED).rotation(-5.0 * self.total_time.as_secs_f32()));

        draw_rect_in_world(ctx, Rect::new(self.controller.body.position.x, self.controller.body.position.y, SIZE, SIZE), graphics::WHITE, &self.world);
        self.tilemap_collider.draw_in_world(ctx, RED, &self.world);

        graphics::present(ctx).expect("Failed to present");
        Ok(())
    }
}

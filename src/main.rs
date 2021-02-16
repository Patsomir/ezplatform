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

use ezplatform::movement::MovementController;
use ezplatform::physics::PhysicsObject;
use ezplatform::world::World;

const SCREEN_WIDTH: f32 = 1200.0;
const SCREEN_HEIGHT: f32 = 600.0;
const DISTANCE: f32 = 15.0;
const RED: Color = Color::new(1.0, 0.0, 0.0, 1.0);

fn main() {
    let (mut ctx, mut event_loop) = ContextBuilder::new("EzPlatform", "Plamen Nikolov")
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .build()
        .unwrap();

    let mut world: World = World::new(SCREEN_WIDTH, SCREEN_HEIGHT, DISTANCE);
    let mut my_game = MyGame::new(&mut ctx, world);

    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

const MASS: f32 = 5.0;
const SIZE: f32 = 2.0;
const MOVE_FORCE: f32 = 6.0;
const JUMP_IMPULSE: f32 = 0.5;
const MAX_SPEED: f32 = 4.0;
const MOVE_SPEED_DECAY: f32 = 3.0;
const GRAVITY_ACCELERATION: f32 = 2.0;

const ZERO_POINT: Point2<f32> = Point2 { x: 0.0, y: 0.0 };

struct MyGame {
    world: World,
    total_time: std::time::Duration,

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

        MyGame {
            world,
            total_time: std::time::Duration::new(0, 0),
            controller,
            can_jump: false,
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

        if self.controller.body.position.x < -DISTANCE * 1.5 {
            self.controller.body.velocity.x = 0.0;
            self.controller.body.position.x = -DISTANCE * 1.5;
        }
        if self.controller.body.position.x > DISTANCE * 1.5 {
            self.controller.body.velocity.x = 0.0;
            self.controller.body.position.x = DISTANCE * 1.5;
        }

        // let camera_pos = self.world.camera_position();
        // self.world.look_at(Point2 { x: camera_pos.x + (self.controller.body.position.x - camera_pos.x) * deltatime.as_secs_f32() * 10.0, y: camera_pos.y });

        if self.controller.body.position.y < -DISTANCE + 2.0 {
            self.controller.body.velocity.y = 0.0;
            self.controller.body.position.y = -DISTANCE + 2.0;
            self.can_jump = true;
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

        draw_rectangle(
            ctx,
            self.world.new_screen_param(
                self.controller.body.position.x,
                self.controller.body.position.y,
                SIZE,
                SIZE,
            ),
        );

        graphics::present(ctx).expect("Failed to present");
        Ok(())
    }
}

fn draw_rectangle(ctx: &mut Context, param: DrawParam) {
    let rect = canonical_rect_mesh(ctx);
    graphics::draw(ctx, &rect, param).unwrap();
}

fn canonical_rect_mesh(ctx: &mut Context) -> Mesh {
    graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        Rect::new(0., 0., 1.0, 1.0),
        graphics::WHITE,
    )
    .unwrap()
}

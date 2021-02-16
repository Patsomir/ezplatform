use ggez::graphics::{Color, Rect};
use ggez::timer;
use ggez::{
    event::{self, EventHandler},
    mint::Point2,
};
use ggez::{graphics, input::keyboard, mint::Vector2};
use ggez::{Context, ContextBuilder, GameResult};
use graphics::{DrawParam, Mesh};

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

const MASS: f32 = 1.0;
const SIZE: f32 = 2.0;
const FORCE: f32 = 1.0;
const ZERO_POINT: Point2<f32> = Point2 { x: 0.0, y: 0.0 };

struct MyGame {
    world: World,
    total_time: std::time::Duration,

    player: PhysicsObject,
}

impl MyGame {
    pub fn new(ctx: &mut Context, world: World) -> MyGame {
        MyGame {
            world,
            total_time: std::time::Duration::new(0, 0),
            player: PhysicsObject::new(ZERO_POINT, MASS),
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let deltatime = timer::delta(ctx);
        self.total_time += deltatime;
        let mut force: Vector2<_> = ZERO_POINT.into();
        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::A) {
            force.x = -FORCE;
        }
        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::D) {
            force.x = FORCE;
        }
        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::W) {
            force.y = FORCE;
        }
        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::S) {
            force.y = -FORCE;
        }
        self.player.set_force(force);
        self.player.update(deltatime);

        if self.player.position.x > DISTANCE {
            self.player.velocity.x = 0.0;
            self.player.position.x = DISTANCE;
        }
        if self.player.position.x < -DISTANCE {
            self.player.velocity.x = 0.0;
            self.player.position.x = -DISTANCE;
        }
        if self.player.position.y < -DISTANCE {
            self.player.velocity.y = 0.0;
            self.player.position.y = -DISTANCE;
        }
        if self.player.position.y > DISTANCE {
            self.player.velocity.y = 0.0;
            self.player.position.y = DISTANCE;
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
            self.world
                .new_screen_param(self.player.position.x, self.player.position.y, SIZE, SIZE),
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

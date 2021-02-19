use std::{collections::HashMap, hash::Hash};

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
use rand::Rng;

const SCREEN_WIDTH: f32 = 1200.0;
const SCREEN_HEIGHT: f32 = 600.0;
const DISTANCE: f32 = 7.0;

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

fn clamp(value: i32, min: i32, max: i32) -> i32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

const E: u32 = 0;
const N: u32 = 1;
const UDLR: u32 = 2;
const UD: u32 = 3;
const LR: u32 = 4;
const ULR: u32 = 5;
const DLR: u32 = 6;
const UDL: u32 = 7;
const UDR: u32 = 8;
const UR: u32 = 9;
const UL: u32 = 10;
const DR: u32 = 11;
const DL: u32 = 12;
const D: u32 = 13;
const R: u32 = 14;
const L: u32 = 15;
const U: u32 = 16;

fn tile_hashmap() -> HashMap<(bool, bool, bool, bool), u32> {
    let mut map: HashMap<(bool, bool, bool, bool), u32> = HashMap::new();
    map.insert((false, false, false, false), N);
    map.insert((false, false, false, true), R);
    map.insert((false, false, true, false), L);
    map.insert((false, false, true, true), LR);
    map.insert((false, true, false, false), D);
    map.insert((false, true, false, true), DR);
    map.insert((false, true, true, false), DL);
    map.insert((false, true, true, true), DLR);
    map.insert((true, false, false, false), U);
    map.insert((true, false, false, true), UR);
    map.insert((true, false, true, false), UL);
    map.insert((true, false, true, true), ULR);
    map.insert((true, true, false, false), UD);
    map.insert((true, true, false, true), UDR);
    map.insert((true, true, true, false), UDL);
    map.insert((true, true, true, true), UDLR);
    map
}

fn generate_ground_template(width: u32, height: u32, start: (u32, u32), end: (u32, u32), limits: (u32, u32), step: u32) -> Vec<Vec<u32>> {
    let mut template: Vec<Vec<u32>> = (0..height).into_iter().map(|_| vec![E; width as usize]).collect();
    let mut floor = start.0 as i32;
    let mut ciel = start.1 as i32;
    let step = step as i32;
    let width = width as i32;
    let height = height as i32;

    for row in 0..height {
        if row < floor || row > ciel {
            template[row as usize][0] = N;
        }
    }

    let mut rng = rand::thread_rng();

    for col in 1..(width - 1) {
        floor = clamp((floor + rng.gen_range(0..=2*step)) - step,0, limits.0 as i32);
        ciel = clamp((ciel + rng.gen_range(0..=2*step)) - step,limits.1 as i32, height - 1);
        if floor == 0 || ciel == height - 1 {
            continue;
        }
        for row in 0..height {
            if row < floor || row > ciel {
                template[row as usize][col as usize] = N;
            }
        }
    }

    floor = end.0 as i32;
    ciel = end.1 as i32;
    for row in 0..height {
        if row < floor - 1 || row > ciel + 1 {
            template[row as usize][(width - 1) as usize] = N;
        }
    }

    let map = tile_hashmap();
    for col in 0..width {
        for row in 0..height {
            if template[row as usize][col as usize] == E {
                continue;
            }
            let sides = (
                row + 1 < height && template[(row + 1) as usize][col as usize] == E,
                row - 1 >= 0 && template[(row - 1) as usize][col as usize] == E,
                col - 1 >= 0 && template[row as usize][(col - 1) as usize] == E,
                col + 1 < width && template[row as usize][(col + 1) as usize] == E,
            );
            template[row as usize][col as usize] = *map.get(&sides).unwrap();
        }
    }

    template.reverse();
    template
}

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
        let body = DynamicCollider::from_rect(Rect::new(ZERO_POINT.x, ZERO_POINT.y,0.98 * SIZE, SIZE), MASS);
        let controller = MovementController::from_components(
            body,
            MOVE_FORCE,
            JUMP_IMPULSE,
            MAX_SPEED,
            MOVE_SPEED_DECAY,
            GRAVITY_ACCELERATION,
            &[Vector2 { x: -0.95, y: -1.2 }, Vector2 { x: 0.95, y: -1.2 }],
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
        let ground_template = generate_ground_template(21, 15, (5, 10), (5, 10), (7,7), 2);
        let ground_template = &ground_template.iter().map(|row| &(*row)[..]).collect::<Vec<_>>()[..];
        let ground = TilemapRenderer::from_components(
            ground_sprites,
            ground_template,
            1.0,
            1.0,
            Point2 { x: 10, y: 7 },
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
            self.controller.collider().position().x,
            self.controller.collider().position().y,
            SIZE,
            SIZE,
        );
        let collisions = self.tilemap_collider.get_collision_lines(player_rect);
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
        self.controller.collider_mut().resolve_collisions(&collisions);
        self.can_jump = false;
        for point in self.controller.ground_check_points().iter() {
            if self.tilemap_collider.check_collision(*point) {
                self.can_jump = true;
                break;
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

        let player_rect = self.controller.rect();
        if player_rect.y < -DISTANCE - 0.5 {
            self.controller.collider_mut().position_mut().y = DISTANCE + 0.5;
        }
        if player_rect.x < -DISTANCE * 2.0 - 0.5 {
            self.controller.collider_mut().position_mut().x = DISTANCE * 2.0 + 0.5;
        }
        if player_rect.x > DISTANCE * 2.0 + 0.5 {
            self.controller.collider_mut().position_mut().x = -DISTANCE * 2.0 - 0.5;
        }

        self.player_animator
            .update(self.controller.collider().velocity(), deltatime);

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
                    self.controller.collider().position().x,
                    self.controller.collider().position().y,
                    self.orientation as f32 * SIZE,
                    SIZE,
                ),
            )
            .unwrap();

        graphics::present(ctx).expect("Failed to present");
        Ok(())
    }
}

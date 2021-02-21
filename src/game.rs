use std::collections::{HashMap, VecDeque};

use ggez::{
    audio::{SoundSource, Source},
    event::{EventHandler, KeyCode, KeyMods},
    graphics::{self, Color, FilterMode, Image, Rect},
    input::keyboard,
    mint::{Point2, Vector2},
    timer, Context, GameResult,
};
use rand::Rng;

use crate::{
    animation::{SpriteAnimator, SpriteSheetAnimation},
    camera::{Camera, FollowDirection, SmoothCamera},
    collision::{DynamicCollider, TilemapCollider},
    movement::MovementController,
    physics::PhysicsObject,
    rendering::{SpriteSheet, TilemapRenderer, WorldDrawable},
    world::World,
};

// Controls
const LEFT_KEY: KeyCode = KeyCode::A;
const RIGHT_KEY: KeyCode = KeyCode::D;
const JUMP_KEY: KeyCode = KeyCode::W;

// Asset paths
const PLAYER_IDLE: &'static str = "/placeholder.png";
const PLAYER_JUMP: &'static str = "/jump.png";
const PLAYER_FALL: &'static str = "/fall.png";
const PLAYER_WALK: &'static str = "/walking.png";
const GROUND_TILES: &'static str = "/ground.png";
const JUMP_SOUND: &'static str = "/jump.wav";

// Player params
const SPAWN_POSITION: Point2<f32> = Point2 { x: 0.0, y: 0.5 };
const MASS: f32 = 3.0;
const PLAYER_WIDTH: f32 = 1.0;
const PLAYER_HEIGHT: f32 = 1.0;
const MOVE_FORCE: f32 = 172.8;
const JUMP_IMPULSE: f32 = 21.6;
const MAX_SPEED: f32 = 11.52;
const MOVE_SPEED_DECAY: f32 = 129.6;
const GRAVITY_ACCELERATION: f32 = 72.0;
const GROUND_CHECK_OFFSETS: &[Vector2<f32>] = &[
    Vector2 { x: -0.98, y: -1.2 },
    Vector2 { x: 0.0, y: -1.2 },
    Vector2 { x: 0.98, y: -1.2 },
];

// Cave params
const TEMPLATE_WIDTH: u32 = 31;
const TEMPLATE_HEIGHT: u32 = 15;
const FLOOR_CEIL_LIMITS: (u32, u32) = (7, 8);
const TEMPLATE_CONNECTIONS: (u32, u32) = (5, 10);
const STEP: u32 = 2;
const TILE_WIDTH: f32 = 1.0;
const TILE_HEIGHT: f32 = 1.0;

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

// Projection params
pub const SCREEN_WIDTH: f32 = 1200.0;
pub const SCREEN_HEIGHT: f32 = 600.0;
const DISTANCE: f32 = 7.0;

// Other params
const BG_COLOR: Color = Color::new(0.1, 0.08, 0.05, 1.0);
const CAMERA_SMOOTHNESS: f32 = 1.5;
const CAMERA_AHEAD_DISTANCE: f32 = 4.0;

struct Player {
    animator: SpriteAnimator<Vector2<f32>>,
    controller: MovementController,
    orientation: i8,
    can_jump: bool,
    jump_sound: Source,
}

impl Player {
    fn new(ctx: &mut Context) -> Self {
        // Controller init
        let body = DynamicCollider::from_rect(
            Rect::new(
                SPAWN_POSITION.x,
                SPAWN_POSITION.y,
                0.98 * PLAYER_WIDTH,
                PLAYER_HEIGHT,
            ),
            MASS,
        );
        let controller = MovementController::from_components(
            body,
            MOVE_FORCE,
            JUMP_IMPULSE,
            MAX_SPEED,
            MOVE_SPEED_DECAY,
            GRAVITY_ACCELERATION,
            GROUND_CHECK_OFFSETS,
        );

        // Animator init
        let idle_image =
            Image::new(ctx, PLAYER_IDLE).expect(&format!("Failed to load {}", PLAYER_IDLE));
        let idle_sprites = SpriteSheet::new(idle_image, 1, 1, 1);
        let idle_animation = SpriteSheetAnimation::new(idle_sprites, 1.0);

        let jump_image =
            Image::new(ctx, PLAYER_JUMP).expect(&format!("Failed to load {}", PLAYER_JUMP));
        let jump_sprites = SpriteSheet::new(jump_image, 1, 1, 1);
        let jump_animation = SpriteSheetAnimation::new(jump_sprites, 1.0);

        let fall_image =
            Image::new(ctx, PLAYER_FALL).expect(&format!("Failed to load {}", PLAYER_FALL));
        let fall_sprites = SpriteSheet::new(fall_image, 1, 1, 1);
        let fall_animation = SpriteSheetAnimation::new(fall_sprites, 1.0);

        let walking_image =
            Image::new(ctx, PLAYER_WALK).expect(&format!("Failed to load {}", PLAYER_WALK));
        let walking_sprites = SpriteSheet::new(walking_image, 3, 2, 6);
        let walking_animation = SpriteSheetAnimation::new(walking_sprites, 30.0);

        let mut animator: SpriteAnimator<Vector2<f32>> = SpriteAnimator::from_animations(vec![
            idle_animation,
            walking_animation,
            jump_animation,
            fall_animation,
        ]);
        animator.add_rule(0, 1, |velocity| velocity.x.abs() > MAX_SPEED / 10.0);
        animator.add_rule(1, 0, |velocity| velocity.x.abs() < MAX_SPEED / 10.0);
        animator.add_rule(0, 2, |velocity| velocity.y > GRAVITY_ACCELERATION / 35.0);
        animator.add_rule(1, 2, |velocity| velocity.y > GRAVITY_ACCELERATION / 35.0);
        animator.add_rule(0, 3, |velocity| velocity.y < -GRAVITY_ACCELERATION / 35.0);
        animator.add_rule(1, 3, |velocity| velocity.y < -GRAVITY_ACCELERATION / 35.0);
        animator.add_rule(2, 3, |velocity| velocity.y <= 0.0);
        animator.add_rule(3, 0, |velocity| velocity.y >= 0.0);

        let jump_sound =
            Source::new(ctx, JUMP_SOUND).expect(&format!("Failed to load {}", JUMP_SOUND));

        Self {
            controller,
            animator,
            orientation: 1,
            can_jump: false,
            jump_sound,
        }
    }

    fn draw(&self, ctx: &mut Context, world: &World) -> GameResult {
        self.animator.get_drawable().draw_in_world(
            ctx,
            world,
            Rect::new(
                self.controller.collider().position().x,
                self.controller.collider().position().y,
                self.orientation as f32 * PLAYER_WIDTH,
                PLAYER_HEIGHT,
            ),
        )
    }
}

struct TilemapCave {
    current_fragment: i32,
    tilemap_renderers: VecDeque<TilemapRenderer>,
    tilemap_colliders: VecDeque<TilemapCollider>,
    tile_hashmap: TileHashmap,
    ground_image: Image,
}

impl TilemapCave {
    fn new(ctx: &mut Context) -> Self {
        let tile_hashmap = tile_hashmap();

        let mut ground_image =
            Image::new(ctx, GROUND_TILES).expect(&format!("Failed to load {}", GROUND_TILES));
        ground_image.set_filter(FilterMode::Nearest);

        let tilemap_renderers: VecDeque<_> = (-1..=1)
            .into_iter()
            .map(|fragment_index| {
                TilemapCave::generate_tilemap_renderer(&ground_image, &tile_hashmap, fragment_index)
            })
            .collect();
        let tilemap_colliders: VecDeque<_> = tilemap_renderers
            .iter()
            .map(|renderer| TilemapCollider::from(renderer))
            .collect();

        Self {
            current_fragment: 0,
            tile_hashmap,
            tilemap_renderers,
            tilemap_colliders,
            ground_image,
        }
    }

    fn draw(&self, ctx: &mut Context, world: &World) -> GameResult {
        for renderer in self.tilemap_renderers.iter() {
            renderer.draw_in_world(ctx, &world, Rect::default())?;
        }
        Ok(())
    }

    fn get_collisions(&self, rect: Rect) -> Vec<Rect> {
        let mut result = Vec::new();
        for collider in self.tilemap_colliders.iter() {
            result.append(&mut collider.get_collision_lines(rect));
        }
        result
    }

    fn check_collision(&self, point: Point2<f32>) -> bool {
        for collider in self.tilemap_colliders.iter() {
            if collider.check_collision(point) {
                return true;
            }
        }
        false
    }

    fn bounds(&self) -> (f32, f32) {
        let tilemap_width = TEMPLATE_WIDTH as f32 * TILE_WIDTH;
        (
            tilemap_width * (self.current_fragment as f32 - 1.5),
            tilemap_width * (self.current_fragment as f32 + 1.5),
        )
    }

    fn push_right(&mut self) {
        self.current_fragment += 1;
        self.tilemap_renderers.pop_front();
        self.tilemap_colliders.pop_front();
        let new_renderer = TilemapCave::generate_tilemap_renderer(
            &self.ground_image,
            &self.tile_hashmap,
            self.current_fragment + 1,
        );
        self.tilemap_colliders
            .push_back(TilemapCollider::from(&new_renderer));
        self.tilemap_renderers.push_back(new_renderer);
    }

    fn push_left(&mut self) {
        self.current_fragment -= 1;
        self.tilemap_renderers.pop_back();
        self.tilemap_colliders.pop_back();
        let new_renderer = TilemapCave::generate_tilemap_renderer(
            &self.ground_image,
            &self.tile_hashmap,
            self.current_fragment - 1,
        );
        self.tilemap_colliders
            .push_front(TilemapCollider::from(&new_renderer));
        self.tilemap_renderers.push_front(new_renderer);
    }

    fn generate_tilemap_renderer(
        image: &Image,
        tile_hashmap: &TileHashmap,
        fragment_index: i32,
    ) -> TilemapRenderer {
        let ground_sprites = SpriteSheet::new(image.clone(), 4, 4, 16);
        let ground_template = generate_ground_template(
            TEMPLATE_WIDTH,
            TEMPLATE_HEIGHT,
            TEMPLATE_CONNECTIONS,
            TEMPLATE_CONNECTIONS,
            FLOOR_CEIL_LIMITS,
            STEP,
            &tile_hashmap,
        );
        let ground_template = &ground_template
            .iter()
            .map(|row| &(*row)[..])
            .collect::<Vec<_>>()[..];
        let tilemap_renderer = TilemapRenderer::from_components(
            ground_sprites,
            ground_template,
            TILE_WIDTH,
            TILE_HEIGHT,
            Point2 {
                x: TEMPLATE_WIDTH as i32 / 2 - fragment_index * TEMPLATE_WIDTH as i32,
                y: TEMPLATE_HEIGHT as i32 / 2,
            },
        );
        tilemap_renderer
    }
}

pub struct EzPlatform {
    world: World,
    camera: SmoothCamera,
    cave: TilemapCave,
    player: Player,
}

impl EzPlatform {
    pub fn new(ctx: &mut Context) -> EzPlatform {
        let player = Player::new(ctx);

        let world: World = World::new(SCREEN_WIDTH, SCREEN_HEIGHT, DISTANCE);

        let mut camera = SmoothCamera::new(world.camera_position(), CAMERA_SMOOTHNESS);
        camera.set_follow_direction(FollowDirection::Horizontal);

        let cave = TilemapCave::new(ctx);

        Self {
            camera,
            world,
            cave,
            player,
        }
    }
}

impl EventHandler for EzPlatform {
    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        if self.player.can_jump && keycode == JUMP_KEY {
            self.player.controller.jump();
            if let Err(_) = self.player.jump_sound.play() {
                println!("Failed to play sound");
            }
            self.player.can_jump = false;
        }
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let deltatime = timer::delta(ctx);
        if keyboard::is_key_pressed(ctx, LEFT_KEY) {
            self.player.controller.move_left();
            self.player.orientation = -1;
        } else if keyboard::is_key_pressed(ctx, RIGHT_KEY) {
            self.player.controller.move_right();
            self.player.orientation = 1;
        } else {
            self.player.controller.stop();
        }
        self.player.controller.update(deltatime);

        let player_rect = self.player.controller.rect();

        let cave_bounds = self.cave.bounds();
        let relative_position_in_cave =
            (player_rect.x - cave_bounds.0) / (cave_bounds.1 - cave_bounds.0);
        if relative_position_in_cave > 0.75 {
            self.cave.push_right();
        } else if relative_position_in_cave < 0.25 {
            self.cave.push_left();
        }

        self.camera.set_destination(Point2 {
            x: player_rect.x + self.player.orientation as f32 * CAMERA_AHEAD_DISTANCE,
            y: player_rect.y,
        });
        self.camera.update(deltatime);
        self.world.look_at(self.camera.position());

        let collisions = self.cave.get_collisions(player_rect);
        self.player
            .controller
            .collider_mut()
            .resolve_collisions(&collisions);
        self.player.can_jump = false;
        for point in self.player.controller.ground_check_points().iter() {
            if self.cave.check_collision(*point) {
                self.player.can_jump = true;
                break;
            }
        }

        if player_rect.y < -DISTANCE - 0.5 {
            self.player.controller.collider_mut().position_mut().y = DISTANCE + 0.5;
        }
        if player_rect.y > DISTANCE + 0.5 {
            self.player.controller.collider_mut().position_mut().y = -DISTANCE - 0.5;
        }

        self.player
            .animator
            .update(self.player.controller.collider().velocity(), deltatime);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, BG_COLOR);

        self.cave.draw(ctx, &self.world)?;

        self.player.draw(ctx, &self.world)?;

        graphics::present(ctx)?;
        Ok(())
    }
}

type TileHashmap = HashMap<(bool, bool, bool, bool), u32>;
fn tile_hashmap() -> TileHashmap {
    let mut map: TileHashmap = HashMap::new();
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

fn clamp(value: i32, min: i32, max: i32) -> i32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

fn generate_ground_template(
    width: u32,
    height: u32,
    start: (u32, u32),
    end: (u32, u32),
    limits: (u32, u32),
    step: u32,
    map: &TileHashmap,
) -> Vec<Vec<u32>> {
    let mut template: Vec<Vec<u32>> = (0..height)
        .into_iter()
        .map(|_| vec![E; width as usize])
        .collect();
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
        floor = clamp(
            (floor + rng.gen_range(0..=2 * step)) - step,
            0,
            limits.0 as i32,
        );
        ciel = clamp(
            (ciel + rng.gen_range(0..=2 * step)) - step,
            limits.1 as i32,
            height - 1,
        );
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
        if row < floor || row > ciel {
            template[row as usize][(width - 1) as usize] = N;
        }
    }

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

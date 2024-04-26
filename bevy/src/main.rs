use bevy::prelude::*;
use bevy_prng::ChaCha8Rng;
use bevy_rand::prelude::*;
use rand::Rng;
use rand_core::SeedableRng;

#[derive(Resource)]
struct Config {
    bh_r: f32,
    angle: f32,
    r: f32,
    da: f32,
    init_ball_number: i32,
    gravity: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bh_r: 100.0,
            angle: 0.0,
            r: 400.0,
            da: std::f32::consts::TAU / 5.0,
            init_ball_number: 8,
            gravity: 100_000_000.0,
        }
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
struct BH {
    s: bool,
}

fn spawn_bhs(mut commands: Commands, asset_server: Res<AssetServer>) {
    let circle: Handle<Image> = asset_server.load("Circle.png");
    commands.spawn((
        BH { s: true },
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.3, 0.3, 0.3),
                ..default()
            },
            texture: circle.clone(),
            ..default()
        },
    ));
    commands.spawn((
        BH { s: false },
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.3, 0.3, 0.3),
                ..default()
            },
            texture: circle,
            ..default()
        },
    ));
}

fn update_angle(mut config: ResMut<Config>, time: Res<Time>) {
    config.angle += config.da * time.delta_seconds();
    if config.angle > std::f32::consts::TAU {
        config.angle -= std::f32::consts::TAU;
    }
}

fn update_bhs(mut bhs: Query<(&BH, &mut Transform)>, config: Res<Config>) {
    for (bh, mut transform) in &mut bhs {
        let pos = Vec2::from_angle(if bh.s {
            config.angle
        } else {
            config.angle - std::f32::consts::PI
        }) * config.bh_r;
        transform.translation.x = pos.x;
        transform.translation.y = pos.y;
    }
}

#[derive(Component)]
struct Ball {
    velocity: Vec2,
}

#[derive(Event)]
struct SpawnBall(Vec2);

fn spawn_ball(
    mut need_spawn_balls: EventReader<SpawnBall>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let asset_handle: Handle<Image> = asset_server.load("Circle.png");
    for pos in need_spawn_balls.read() {
        commands.spawn((
            Ball {
                velocity: Vec2::ZERO,
            },
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(1.0, 1.0, 1.0),
                    ..default()
                },
                texture: asset_handle.clone(),
                transform: Transform::from_translation(pos.0.extend(0.0))
                    .with_scale(Vec3::splat(0.4)),
                ..default()
            },
        ));
    }
}

fn spawn_init_balls(
    mut ball_spawner: EventWriter<SpawnBall>,
    config: Res<Config>,
    mut rng: ResMut<GlobalEntropy<ChaCha8Rng>>,
) {
    for _ in 0..config.init_ball_number {
        ball_spawner.send(SpawnBall(
            Vec2::from_angle(rng.gen_range(0.0..std::f32::consts::TAU)) * config.r,
        ));
    }
}

fn update_balls(
    time: Res<Time>,
    config: Res<Config>,
    mut balls: Query<(Entity, &mut Ball, &mut Transform)>,
    bhs: Query<&Transform, (With<BH>, Without<Ball>)>,
    mut commands: Commands,
    mut ball_spawner: EventWriter<SpawnBall>,
    mut rng: ResMut<GlobalEntropy<ChaCha8Rng>>,
) {
    for (entity, mut ball, mut transform) in &mut balls {
        let a: Vec2 = bhs
            .iter()
            .map(|bh| {
                let direction = (bh.translation - transform.translation).xy();
                config.gravity * direction.normalize() / direction.length_squared()
            })
            .sum();
        let dv: Vec2 = a * time.delta_seconds();
        ball.velocity += dv;
        transform.translation.x += ball.velocity.x * time.delta_seconds();
        transform.translation.y += ball.velocity.y * time.delta_seconds();

        if bhs
            .iter()
            .any(|bh| bh.translation.distance(transform.translation) <= 32.0 * 1.4 / 2.0)
        {
            commands.entity(entity).despawn();
            ball_spawner.send(SpawnBall(
                Vec2::from_angle(rng.gen_range(0.0..std::f32::consts::TAU)) * config.r,
            ));
            ball_spawner.send(SpawnBall(
                Vec2::from_angle(rng.gen_range(0.0..std::f32::consts::TAU)) * config.r,
            ));
        }
    }
}

fn log_info(query: Query<With<Ball>>, time: Res<Time>) {
    println!(
        "FPS: {:4.0} Ball number: {:7}",
        1.0 / time.delta_seconds(),
        query.iter().count()
    );
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Config::default())
        .insert_resource(GlobalEntropy::<ChaCha8Rng>::new(SeedableRng::from_entropy()))
        .add_event::<SpawnBall>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_bhs)
        .add_systems(Startup, spawn_init_balls)
        .add_systems(Startup, update_bhs)
        .add_systems(First, update_angle)
        .add_systems(StateTransition, update_bhs)
        .add_systems(StateTransition, update_balls)
        .add_systems(SpawnScene, spawn_ball)
        .add_systems(Update, log_info)
        .run();
}

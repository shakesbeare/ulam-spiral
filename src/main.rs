use ::bevy::prelude::*;

const WALL: f32 = 100.;

fn main() {
    App::new()
        .insert_resource(NumCounter(1))
        .insert_resource(NextCoord::default())
        .insert_resource(CountUntilTurn::default())
        .insert_resource(CurrentDirection::default())
        .insert_resource(FaceProgressCounter::default())
        .insert_resource(TurnsDone::default())
        .add_startup_system(setup)
        .insert_resource(ClearColor(Color::hex("1d2021").unwrap()))
        .add_system(spawn_dots)
        .add_plugins(DefaultPlugins)
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(position_translation)
                .with_system(size_scaling),
        )
        .run();
}

#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Size {
    width: f32,
    height: f32,
}

impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

struct NumCounter(i64);

#[derive(Default)]
struct NextCoord(Position);

#[derive(Default)]
struct CountUntilTurn(i64);

#[derive(Default)]
struct FaceProgressCounter(i64);

#[derive(Default)]
struct CurrentDirection(Direction);

#[derive(Component)]
struct Dot;

#[derive(Default)]
struct TurnsDone(i64);

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Right
    }
}

impl Direction {
    fn next(&self) -> Self {
        let next = match self {
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Up => Direction::Left,
        };

        return next;
    }
}

fn setup(mut commands: Commands, mut next_coord: ResMut<NextCoord>) {
    next_coord.0.x = (WALL / 2.) as i32;
    next_coord.0.y = (WALL / 2.) as i32;
    commands.spawn_bundle(Camera2dBundle::default());
}

fn spawn_dots(
    mut commands: Commands,
    mut num_counter: ResMut<NumCounter>,
    mut next_coord: ResMut<NextCoord>,
    mut count_until_turn: ResMut<CountUntilTurn>,
    mut current_direction: ResMut<CurrentDirection>,
    mut face_progress_counter: ResMut<FaceProgressCounter>,
    mut turns_done: ResMut<TurnsDone>,
) {
    if is_prime(num_counter.0) {
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    ..default()
                },
                ..default()
            })
            .insert(Dot)
            .insert(Position { x: next_coord.0.x, y: next_coord.0.y })
            .insert(Size::square(0.8));
    }

    if face_progress_counter.0 >= count_until_turn.0 {
        current_direction.0 = current_direction.0.next();
        face_progress_counter.0 = 0;
        turns_done.0 += 1;
    } else {
        face_progress_counter.0 += 1;
    }

    if turns_done.0 >= 2 {
        count_until_turn.0 += 1;
        turns_done.0 = 0;
    }

    println!("{:?}", current_direction.0);
    match current_direction.0 {
        Direction::Up => {
            next_coord.0.y += 1;
        }
        Direction::Right => {
            next_coord.0.x += 1;
        }
        Direction::Down => {
            next_coord.0.y -= 1;
        }
        Direction::Left => {
            next_coord.0.x -= 1;
        }
    }

    num_counter.0 += 1;
}

fn is_prime(num: i64) -> bool {
    let limit = f64::sqrt(num as f64);
    for divisor in 2..limit as i64 {
        if num % divisor == 1 {
            return false;
        }
    }

    return true;
}

fn size_scaling(windows: Res<Windows>, mut query: Query<(&Size, &mut Transform)>) {
    let window = windows.get_primary().unwrap();
    for (sprite_size, mut transform) in query.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / WALL as f32
                * window.width() as f32
                * (window.height() / window.width()),
            sprite_size.height / WALL as f32 * window.height() as f32,
            1.,
        )
    }
}

fn position_translation(windows: Res<Windows>, mut query: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }

    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in query.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, WALL as f32),
            convert(pos.y as f32, window.height() as f32, WALL as f32),
            0.,
        )
    }
}

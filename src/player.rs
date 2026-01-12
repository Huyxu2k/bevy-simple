use bevy::prelude::*;

// atlas constants
const TILE_SIZE: u32 = 64;
const WALK_FRAMES: usize = 9;
const MOVE_SPEED: f32 = 140.0;
const ANIM_DT: f32 = 0.1;

const PLAYER_Z: f32 = 20.0;

// player plugin
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (move_player, animate_player));
    }
}

// object
#[derive(Component)]
struct Player;

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq)]
enum Facing {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct AnimationState {
    facing: Facing,
    moving: bool,
    was_moving: bool,
}

// behavior function
fn move_player(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player: Query<(&mut Transform, &mut AnimationState), With<Player>>
) {
    let Ok((mut transform, mut animation)) = player.single_mut() else {
        return;
    };
    let mut direction = Vec2::ZERO;

    if input.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    } else if input.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    } else if input.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    } else if input.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }

    if direction != Vec2::ZERO {
        let delta = direction.normalize() * MOVE_SPEED * time.delta_secs();
        transform.translation.x += delta.x;
        transform.translation.y += delta.y;
        animation.moving = true;

        if direction.x.abs() > direction.y.abs() {
            animation.facing = if direction.x > 0.0 { Facing::Right } else { Facing::Left };
        } else {
            animation.facing = if direction.y > 0.0 { Facing::Up } else { Facing::Down };
        }
    } else {
        animation.moving = false;
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
){
    let texture= asset_server.load::<Image>("male_character-spritesheet.png");
    let layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(TILE_SIZE), 
        WALK_FRAMES as u32, 
        12, 
        None, 
        None,
    ));

    let facing= Facing::Down;
    let start_index = atlas_index_for(facing, 0);

    commands.spawn((
        Sprite::from_atlas_image(
            texture, 
            TextureAtlas { layout, index: start_index }
        ),
        Transform::from_translation(Vec3::new(0., 0., PLAYER_Z)).with_scale(Vec3::splat(0.8)),
        Player,
        AnimationState { facing, moving: false, was_moving: false },
        AnimationTimer(Timer::from_seconds(ANIM_DT, TimerMode::Repeating)),
    ));
}

fn animate_player(
    time: Res<Time>,
    mut query: Query<(&mut AnimationState, &mut AnimationTimer, &mut Sprite), With<Player>>
) {
    let Ok((mut animation, mut timer, mut sprite)) = query.single_mut() else {
        return;;
    };

    let atlas = match  sprite.texture_atlas.as_mut() {
        Some(result) => result,
        None => return,
    };

    let target_row = row_zero_based(animation.facing);
    let mut current_col = atlas.index  % WALK_FRAMES;
    let mut current_row = atlas.index / WALK_FRAMES;

    if current_row != target_row {
        atlas.index = row_start_index(animation.facing);
        current_col = 0;
        current_row = target_row;
        timer.reset();
    }    

    let just_started = animation.moving && !animation.was_moving;
    let just_stopped = !animation.moving && animation.was_moving;

    if animation.moving{
        if just_started {
            let row_start = row_start_index(animation.facing);
            let next_col = (current_col + 1) % WALK_FRAMES;
            atlas.index = row_start + next_col;
            timer.reset();
        }else{
            timer.tick(time.delta());
            if timer.just_finished() {
                let row_start = row_start_index(animation.facing);
                let next_col = (current_col + 1) % WALK_FRAMES;
                atlas.index = row_start + next_col;
            }
        }
    }else if just_stopped{
        timer.reset();
    }

    animation.was_moving = animation.moving;
}

// help function
fn row_zero_based(facing: Facing) -> usize {
    match facing {
        Facing::Up => 8,
        Facing::Left => 9,
        Facing::Down => 10,
        Facing::Right => 11,
    }
}
fn row_start_index(facing: Facing) -> usize {
    row_zero_based(facing) * WALK_FRAMES
}
fn atlas_index_for(
    facing: Facing,
    frame_in_row: usize
) -> usize{
    row_start_index(facing) + frame_in_row.min(WALK_FRAMES - 1)
}

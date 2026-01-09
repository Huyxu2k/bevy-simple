use bevy::prelude::*;

// Atlas constants
const TITLE_SIZE: u32 = 64;
const WALK_FRAMES: usize = 9;
const MOVE_SPEED: f32 = 140.0;
const ANIM_DT: f32 = 0.1;

#[derive(Component)]
pub struct Player;

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

pub fn move_player(
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

pub fn animate_player(
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

    
}

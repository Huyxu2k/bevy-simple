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
    Right
}

pub fn move_player(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_transform: Single<&mut Transform, With<Player>>,
) {
    let mut direction = Vec2::ZERO;
    
    if input.pressed(KeyCode::ArrowLeft){
        direction.x -= 1.0;
    }
    else if input.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }
    else if input.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
     else if input.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }

    if direction != Vec2::ZERO{
        let speed = 300.0;
        let delta = direction.normalize() * speed * time.delta_secs();
        player_transform.translation.x += delta.x;
        player_transform.translation.y += delta.y;
    }
}

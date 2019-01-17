use super::super::geometry::Vec2D;

use super::GameState;
use super::{Input, Key};

pub fn input_system(game_state: &mut GameState) {
    if let Some(input) = &game_state.input {
        if let Some(player) = &mut game_state.entities[game_state.player] {
            if let Some(physics) = &mut player.physics {
                physics.position += get_velocity(&input, physics.speed, game_state.time_elapsed);
            }
        }
    }
}

fn get_velocity(input: &Input, speed: f64, time_elapsed: f64) -> Vec2D {
    let mut velocity = Vec2D::new(0.0, 0.0);
    if input.key_pressed.contains(&Key::UpArrow) {
        velocity += Vec2D::new(0.0, -speed);
    }

    if input.key_pressed.contains(&Key::DownArrow) {
        velocity += Vec2D::new(0.0, speed);
    }

    if input.key_pressed.contains(&Key::LeftArrow) {
        velocity += Vec2D::new(-speed, 0.0);
    }

    if input.key_pressed.contains(&Key::RightArrow) {
        velocity += Vec2D::new(speed, 0.0);
    }

    velocity * time_elapsed
}

#[test]
fn test_input_system() {}

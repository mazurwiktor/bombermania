use super::GameState;
use super::call;

pub fn physics_system(game_state: &mut GameState) {
    for entity in &mut game_state.entities {
        if let Some(entity) = entity {
            if let Some(physics) = &mut entity.physics {
                physics.position += physics.velocity;
            }
        }
    }
}

pub fn render_system(game_state: &GameState) {
    call::clear_screen();

    for entity in &game_state.entities {
        if let Some(entity) = entity {
            if let Some(physics) = &entity.physics {
                call::draw_player(physics.position);
            }
        }
    }
}
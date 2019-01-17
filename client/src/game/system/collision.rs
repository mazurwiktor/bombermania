use super::super::geometry::Point;

use super::GameState;

pub fn collision_system(game_state: &mut GameState) {
    let game_size = game_state.size;
    for mut entity in &mut game_state.entities {
        if let Some(entity) = &mut entity {
            if let Some(physics) = &mut entity.physics {
                let (width, height) = (20.0, 20.0);
                if physics.position.x + width >= game_size.width {
                    physics.position = Point::new(game_size.width - width, physics.position.y);
                }

                if physics.position.x <= 0.0 {
                    physics.position = Point::new(0.0, physics.position.y);
                }

                if physics.position.y + height >= game_size.height {
                    physics.position = Point::new(physics.position.x, game_size.height - height);
                }

                if physics.position.y <= 0.0 {
                    physics.position = Point::new(physics.position.x, 0.0);
                }
            }
        }
    }
}

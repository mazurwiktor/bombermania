use specs::{Read, ReadStorage, WriteStorage, System};

use engine::components;
use engine::resources;

pub struct Movement;

impl<'a> System<'a> for Movement {
    type SystemData = (
        Read<'a, resources::input::InputState>,
        ReadStorage<'a, components::identity::Identity>,
        WriteStorage<'a, components::physics::Position>);

    fn run(&mut self, (input_states, identity, mut position): Self::SystemData) {
        use specs::Join;
        info!("SYSTEM Movement");
        for (ent_id, ent_pos) in (&identity, &mut position).join() {
            info!("{}", &ent_id.id);
            info!("├── {:?}", &ent_pos);

            let opt_ent_is = input_states.content.get(&ent_id.id);
            if let Some(ent_is) = opt_ent_is {
                info!("└── {:?}", &ent_is);
                // TODO:
                // 1. cleanup
                // 2. map boundary check
                // 3. velocity, to avoid reaching boundary in few CPU ticks
                if ent_is.up { ent_pos.y += 1; };
                if ent_is.down { ent_pos.y -= 1; };
                if ent_is.left { ent_pos.x -= 1; };
                if ent_is.right { ent_pos.x += 1; };
            }
            else {
                warn!("└── no InputState!");
            }

        }
    }
}

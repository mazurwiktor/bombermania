use specs::{Read, ReadStorage, WriteStorage, System};

use engine::components;
use engine::resources;

pub struct Movement;

impl<'a> System<'a> for Movement {
    type SystemData = (
        Read<'a, resources::input::InputState>,
        Read<'a, resources::deltatime::DeltaTime>,
        ReadStorage<'a, components::identity::Identity>,
        ReadStorage<'a, components::physics::Velocity>,
        WriteStorage<'a, components::physics::Position>);

    fn run(&mut self, (
            input_states,
            deltatime,
            identity,
            velocity,
            mut position): Self::SystemData)
    {
        use specs::Join;
        info!("SYSTEM Movement");
        for (ent_id, ent_vel, ent_pos) in (&identity, &velocity, &mut position).join() {
            info!("{}", &ent_id.id);
            info!("├── {:?}", &ent_vel);
            info!("├── {:?}", &ent_pos);

            let opt_ent_is = input_states.content.get(&ent_id.id);
            if let Some(ent_is) = opt_ent_is {
                info!("└── {:?}", &ent_is);

                // TODO: HACK, as_float_secs() does not compile for whatever reason on rust 1.34
                //let dt = deltatime.content.as_float_secs();
                let dt = deltatime.content.as_secs() as f64 +
                         deltatime.content.subsec_micros() as f64 / 1_000_000.0;
                // TODO:
                // 1. cleanup
                // 2. map boundary check
                if ent_is.up { ent_pos.y += ent_vel.y * dt };
                if ent_is.down { ent_pos.y -= ent_vel.y * dt; };
                if ent_is.left { ent_pos.x -= ent_vel.x * dt; };
                if ent_is.right { ent_pos.x += ent_vel.x * dt; };
            }
            else {
                warn!("└── no InputState!");
            }

        }
    }
}

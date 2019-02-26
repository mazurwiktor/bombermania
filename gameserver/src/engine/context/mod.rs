use specs;

use specs::{Builder, RunNow};

use engine::components;
use engine::interface;
use engine::resources;
use engine::systems;
use engine::types;

pub struct InputEvent {
    pub id: types::Id,  // which player
    pub content: interface::Input  // active keystrokes
}

pub enum CommonEvent {
    Join(types::Id)
}

pub struct Context<'a> {
    world: specs::World,
    dispatcher: specs::Dispatcher<'a, 'a>
}

impl<'a> Context<'a> {
    pub fn new() -> Self {
        let mut world = specs::World::new();
        world.register::<components::identity::Identity>();
        world.register::<components::physics::Position>();

        world.add_resource(resources::input::InputState::new());

        let mut sys_movement = systems::physics::Movement;
        sys_movement.run_now(&world.res);

        let dispatcher = specs::DispatcherBuilder::new()
            .with(systems::physics::Movement, "sys_movement", &[])
            .build();

        Context{world, dispatcher}
    }

    /// Handles events like:
    /// * join player
    /// * remove player
    /// * options (?)
    /// * chat (?)
    pub fn evt_common(&mut self, evt: &CommonEvent) {
        match evt {
            CommonEvent::Join(id) => self.add_player(id, 1, 1)
        }
    }

    /// Handles player input (keystrokes) events
    pub fn evt_input(&mut self, evt: &InputEvent) {
        debug!("evt!");
        // TODO: obviously I need to learn about borrowing system and how to drop borrow.
        {
            let mut input_resource = self.world.write_resource::<resources::input::InputState>();
            // TODO: is it possible to update map without `entry + or_insert` idiom?
            let entry = input_resource.content.entry(evt.id.clone())
                .or_insert(interface::Input::new());
            *entry = evt.content.clone();  // TODO: possibly clone can be replaced with smth
        }
        self.dispatcher.dispatch(&mut self.world.res);
    }

    fn add_player(&mut self, id: &types::Id, x: u32, y: u32) {
        self.world.create_entity()
            .with(components::identity::Identity{ id: id.clone() })
            .with(components::physics::Position{ x: x, y: y })
            .build();
        self.world.maintain();
    }
}
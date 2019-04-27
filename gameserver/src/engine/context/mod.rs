use specs;

use specs::{Builder, RunNow};

use engine::components;
use engine::interface;
use engine::resources;
use engine::systems;
use engine::types;

static VELOCITY: f64 = 0.5;  // standard "unboosted" velocity [tiles per second]

pub struct InputEvent {
    pub id: types::Id,  // which player
    pub content: interface::Input  // active keystrokes
}

pub enum Event {
    Join(types::Id),
    Leave(types::Id),
    Input(InputEvent),
    Tick(types::Period)
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
        world.register::<components::physics::Velocity>();

        world.add_resource(resources::input::InputState::new());
        world.add_resource(resources::deltatime::DeltaTime::new());

        let mut sys_movement = systems::physics::Movement;
        sys_movement.run_now(&world.res);

        let dispatcher = specs::DispatcherBuilder::new()
            .with(systems::physics::Movement, "sys_movement", &[])
            .build();

        Context{world, dispatcher}
    }

    pub fn evt(&mut self, evt: &Event) {
        match evt {
            Event::Join(id) => self.add_player(id, 1.0, 1.0),  // TODO: spawn coords deducer
            Event::Leave(id) => info!("lol, {} left", id),
            Event::Input(ievt) => self.input(ievt),
            Event::Tick(period) => self.tick(period)
        }
    }

    fn input(&mut self, evt: &InputEvent) {
        info!("input evt");
        let mut input_resource = self.world.write_resource::<resources::input::InputState>();
        // TODO: is it possible to update map without `entry + or_insert` idiom?
        let entry = input_resource.content.entry(evt.id.clone())
            .or_insert(interface::Input::new());
        *entry = evt.content.clone();  // TODO: possibly clone can be replaced with smth
    }

    fn add_player(&mut self, id: &types::Id, x: f64, y: f64) {
        info!("Spawning [{}] on x:{} y:{}", id, &x, &y);
        self.world.create_entity()
            .with(components::identity::Identity{ id: id.clone() })
            .with(components::physics::Velocity{ x: VELOCITY, y: VELOCITY })
            .with(components::physics::Position{ x: x, y: y })
            .build();
        self.world.maintain();
    }

    fn tick(&mut self, period: &types::Period) {
        {
            let mut deltatime = self.world.write_resource::<resources::deltatime::DeltaTime>();
            *deltatime = resources::deltatime::DeltaTime::from(period);
            info!("TICK {:?}", period);
        }
        self.dispatcher.dispatch(&mut self.world.res);
    }
}

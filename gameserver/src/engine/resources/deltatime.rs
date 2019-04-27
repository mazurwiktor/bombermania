use engine;

#[derive(Default)]
pub struct DeltaTime {
    pub content: engine::types::Period
}

impl DeltaTime {
    pub fn new() -> Self {
        DeltaTime {
            content: engine::types::Period::new(0, 0)
        }
    }

    pub fn from(period: &engine::types::Period) -> Self {
        DeltaTime {
            content: period.clone()
        }
    }
}

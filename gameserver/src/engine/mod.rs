pub mod components;
pub mod context;
pub mod interface;
pub mod resources;
pub mod systems;

pub mod types {
    pub type Id = snowflake::ProcessUniqueId;
    pub type Period = std::time::Duration;
}

pub mod bundles;
pub mod components;
pub mod constants;
pub mod plugins;
pub mod resources;
pub mod systems;

/// Re-export the most used pieces for ergonomic `use mygame::prelude::*;`
pub mod prelude {
    pub use crate::bundles::*;
    pub use crate::components::*;
    pub use crate::constants::*;
    pub use crate::plugins::*;
    pub use crate::resources::*;
    pub use crate::systems::*;
}

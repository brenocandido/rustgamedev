pub mod constants;
pub mod components;
pub mod bundles;
pub mod systems;

/// Re-export the most used pieces for ergonomic `use mygame::prelude::*;`
pub mod prelude {
    pub use crate::components::*;
    pub use crate::bundles::*;
    pub use crate::constants::*;
    pub use crate::systems::*;
}
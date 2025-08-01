
pub mod input;
pub mod physics;
pub mod spawn;
pub mod core;
pub mod attributes;
pub mod combat;
pub mod behavior;

pub mod prelude {
    pub use bevy::prelude::*;
    pub use bevy_inspector_egui::prelude::*;

    pub use crate::input::plugin::*;
    pub use crate::input::components::*;

    pub use crate::physics::components::*;
    pub use crate::physics::plugin::*;
    pub use crate::physics::bundles::*;
    pub use crate::physics::collision::*;
    pub use crate::physics::resources::*;

    pub use crate::spawn::bundles::*;
    pub use crate::spawn::plugin::*;

    pub use crate::core::bundles::*;
    pub use crate::core::components::*;
    pub use crate::core::plugin::*;
    pub use crate::core::resources::*;

    pub use crate::core::inspector::plugin::*;

    pub use crate::attributes::components::*;

    pub use crate::combat::plugin::*;
    pub use crate::combat::events::*;

    pub use crate::behavior::components::*;
    pub use crate::behavior::plugin::*;
}

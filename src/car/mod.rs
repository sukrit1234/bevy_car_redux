mod car;
mod esp;
mod joint;
mod spawn;
mod spec;
mod wheel;
mod sensor;
mod dash;
mod network;
pub mod control;

pub use network::*;
pub use car::*;
pub use esp::*;
pub use spec::*;
pub use wheel::*;
pub use spawn::*;
pub use dash::*;
pub use control::*;

use bevy::prelude::SystemSet;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub enum CarSet {
    Input,
    NeuralNetwork,
    Esp,
}

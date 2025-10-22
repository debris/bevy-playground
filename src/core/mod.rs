mod press;
mod mouse;
mod touch;

pub mod prelude {
    pub use super::mouse::{MousePlugin, MousePosition};
    pub use super::touch::{TouchPlugin, TouchArea, TouchState, just_touched};
    pub use super::press::{PressPlugin, PressArea, PressState};
}


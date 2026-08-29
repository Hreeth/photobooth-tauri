mod session;
mod types;

use std::sync::Mutex;

pub use session::{Options, Session, SessionStatus};
pub use types::{FilterKind, LayoutKind};

use crate::camera::Camera;

pub struct CameraState {
    pub driver: Box<dyn Camera>,
    pub is_streaming: bool,
}

impl Default for CameraState {
    fn default() -> Self {
        Self { driver: crate::camera::new(), is_streaming: false }
    }
}

pub struct AppState {
    pub session: Mutex<Session>,
    pub camera: Mutex<CameraState>,
}

impl Default for AppState {
    fn default() -> Self {
        Self { session: Mutex::new(Session::default()), camera: Mutex::new(CameraState::default()) }
    }
}

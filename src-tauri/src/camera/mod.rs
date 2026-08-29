#[cfg(windows)]
mod mock;
#[cfg(any(target_os = "linux", all(windows, debug_assertions)))]
mod rpi;

#[cfg(windows)]
pub use mock::MockCamera;
#[cfg(target_os = "linux")]
pub use rpi::RpiCamera;
use tauri::AppHandle;

use crate::Result;

pub trait Camera: Send {
    fn start_stream(&mut self, app: AppHandle) -> Result<()>;
    fn stop_stream(&mut self) -> Result<()>;
    fn capture(&mut self) -> Result<String>;
}

pub fn new() -> Box<dyn Camera> {
    #[cfg(target_os = "linux")]
    {
        use crate::camera::RpiCamera;
        Box::new(RpiCamera::new())
    }

    #[cfg(windows)]
    {
        use crate::camera::MockCamera;
        Box::new(MockCamera::new())
    }
}

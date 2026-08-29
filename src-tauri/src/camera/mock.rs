use std::{
    path::PathBuf,
    sync::{Arc, atomic::AtomicBool},
    time::Duration,
};

use base64::Engine;
use tauri::Emitter;

use crate::{Result, camera::Camera, utils::capture_dir};

pub struct MockCamera {
    sample_path: PathBuf,
    running: Arc<AtomicBool>,
    thread: Option<std::thread::JoinHandle<()>>,
    index: usize,
}

impl MockCamera {
    pub fn new() -> Self {
        Self {
            sample_path: PathBuf::from("sample.jpg"),
            running: Arc::new(AtomicBool::new(false)),
            thread: None,
            index: 0,
        }
    }
}

impl Camera for MockCamera {
    fn start_stream(&mut self, app: tauri::AppHandle) -> Result<()> {
        if self.running.swap(true, std::sync::atomic::Ordering::SeqCst) {
            return Ok(());
        }

        let sample_path = self.sample_path.clone();
        let running = Arc::clone(&self.running);

        self.thread = Some(std::thread::spawn(move || {
            let frame = match std::fs::read(&sample_path) {
                Ok(frame) => frame,
                Err(e) => {
                    eprintln!("[mock-camera] failed to read sample image: {e}");

                    running.store(false, std::sync::atomic::Ordering::SeqCst);
                    return;
                }
            };

            let b64 = base64::engine::general_purpose::STANDARD.encode(&frame);

            while running.load(std::sync::atomic::Ordering::SeqCst) {
                let _ = app.emit("camera-frame", &b64);

                std::thread::sleep(Duration::from_millis(33));
            }

            println!("[mock-camera] stream stopped");
        }));

        Ok(())
    }

    fn stop_stream(&mut self) -> Result<()> {
        self.running.store(false, std::sync::atomic::Ordering::SeqCst);

        if let Some(thread) = self.thread.take() {
            thread.join().map_err(|_| "failed to stop mock camera thread".to_string())?;
        }

        Ok(())
    }

    fn capture(&mut self) -> Result<String> {
        std::fs::create_dir_all(capture_dir()).map_err(|e| e.to_string())?;

        let output_path = capture_dir().join(format!("img_{:04}.jpg", self.index));

        self.index += 1;

        std::fs::copy(&self.sample_path, &output_path)
            .map_err(|e| format!("failed to copy sample image: {e}"))?;

        Ok(output_path.to_string_lossy().to_string())
    }
}

use std::{
    io::{BufRead, BufReader, Read},
    process::{Child, Command, Stdio},
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread,
};

use base64::Engine;
use tauri::Emitter;

use crate::{Result, camera::Camera, utils::capture_dir};

pub struct RpiCamera {
    process: Option<Child>,
    latest_frame: Arc<Mutex<Option<Vec<u8>>>>,
    running: Arc<AtomicBool>,
    index: usize,
}

impl RpiCamera {
    pub fn new() -> Self {
        Self {
            process: None,
            latest_frame: Arc::new(Mutex::new(None)),
            running: Arc::new(AtomicBool::new(false)),
            index: 0,
        }
    }
}

impl Camera for RpiCamera {
    fn start_stream(&mut self, app: tauri::AppHandle) -> Result<()> {
        if self.process.is_some() {
            return Ok(());
        }

        #[rustfmt::skip]
        let mut child = Command::new("rpicam-vid")
            .args([
                "--width", "2028",
                "--height", "1520",
                "--framerate", "30",
                "--codec", "mjpeg",
                "--inline",
                "--nopreview",
                "--timeout", "0",
                "--hflip",
                "--denoise", "cdn_off",
                "--shutter", "18000",
                "--gain", "10",
                "--ev", "0",
                "--hdr", "auto",
                "--quality", "95",
                "--roi", "0.075,0.15,0.79,0.85",
                "--output",
                "-",
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("failed to start rpicam-vid: {e}"))?;

        let mut stdout = child.stdout.take().ok_or("failed to access camera stdout")?;

        let stderr = child.stderr.take().ok_or("failed to access camera stderr")?;

        let latest_frame = Arc::clone(&self.latest_frame);
        let running = Arc::clone(&self.running);

        running.store(true, Ordering::SeqCst);

        thread::spawn(move || {
            let reader = BufReader::new(stderr);

            for line in reader.lines() {
                match line {
                    Ok(line) if !line.trim().is_empty() => {
                        eprintln!("[rpicam-vid] {line}");
                    }
                    Ok(_) => {}
                    Err(_) => break,
                }
            }
        });

        thread::spawn(move || {
            let mut buffer = Vec::new();

            while running.load(Ordering::SeqCst) {
                let mut chunk = [0u8; 4096];

                match stdout.read(&mut chunk) {
                    Ok(n) if n > 0 => {
                        buffer.extend_from_slice(&chunk[..n]);

                        while let Some(frame) = extract_jpeg(&mut buffer) {
                            {
                                let mut guard = latest_frame.lock().unwrap();
                                *guard = Some(frame.clone());
                            }

                            let b64 = base64::engine::general_purpose::STANDARD.encode(&frame);

                            let _ = app.emit("camera-frame", b64);
                        }
                    }

                    _ => break,
                }
            }
        });

        self.process = Some(child);

        Ok(())
    }

    fn stop_stream(&mut self) -> Result<()> {
        self.running.store(false, Ordering::SeqCst);

        if let Some(mut process) = self.process.take() {
            let _ = process.kill();
            let _ = process.wait();
        }

        *self.latest_frame.lock().unwrap() = None;

        Ok(())
    }

    fn capture(&mut self) -> Result<String> {
        let frame = {
            let guard = self.latest_frame.lock().unwrap();

            guard.clone().ok_or_else(|| "no frame available yet".to_string())?
        };

        if frame.is_empty() {
            return Err("empty frame".into());
        }

        std::fs::create_dir_all(capture_dir())
            .map_err(|e| format!("failed to create capture directory: {e}"))?;

        let output_path = capture_dir().join(format!("img_{:04}.jpg", self.index));

        self.index += 1;

        std::fs::write(&output_path, frame).map_err(|e| format!("failed to save image: {e}"))?;

        Ok(output_path.to_string_lossy().to_string())
    }
}

fn extract_jpeg(buffer: &mut Vec<u8>) -> Option<Vec<u8>> {
    let start = buffer.windows(2).position(|w| w == [0xFF, 0xD8])?;

    let end = buffer.windows(2).position(|w| w == [0xFF, 0xD9])?;

    if end > start {
        let frame = buffer[start..end + 2].to_vec();
        buffer.drain(..end + 2);
        Some(frame)
    } else {
        None
    }
}

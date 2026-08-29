use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    Result,
    state::{FilterKind, LayoutKind},
};

const MAX_RETAKES: u8 = 2;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Options {
    pub prints: u8,
    pub digital: bool,
    pub layout: LayoutKind,
    pub filter: FilterKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SessionStatus {
    Waiting,
    Running,
    Reviewing,
    Completed,
}

#[derive(Debug)]
pub struct Session {
    pub session_id: Uuid,

    pub status: SessionStatus,

    pub photos: Vec<PathBuf>,
    pub current_preview: Option<PathBuf>,

    pub options: Option<Options>,

    pub current_take: usize,
    pub retakes: u8,

    pub r#final: Option<PathBuf>,
}

impl Default for Session {
    fn default() -> Self {
        Self {
            session_id: Uuid::new_v4(),
            status: SessionStatus::Waiting,
            photos: Vec::new(),
            current_preview: None,
            options: None,
            current_take: 0,
            retakes: 0,
            r#final: None,
        }
    }
}

impl Session {
    pub fn start(&mut self, options: Options) {
        self.status = SessionStatus::Running;
        self.options = Some(options);
        self.current_take = 0;
        self.retakes = 0;
        self.photos.clear();
        self.current_preview = None;
        self.r#final = None;
    }

    pub fn can_capture(&self) -> bool {
        self.status == SessionStatus::Running
    }

    pub fn set_preview(&mut self, path: PathBuf) {
        self.current_preview = Some(path);
        self.status = SessionStatus::Reviewing;
    }

    pub fn can_retake(&self) -> bool {
        self.status == SessionStatus::Reviewing && self.retakes < MAX_RETAKES
    }

    pub fn retake(&mut self) -> Result<PathBuf> {
        if !self.can_retake() {
            return Err("maximum retakes reached".into());
        }

        let preview = self.current_preview.take().ok_or("no photo to retake")?;

        self.retakes += 1;
        self.status = SessionStatus::Running;

        Ok(preview)
    }

    pub fn accept_preview(&mut self) -> Result<()> {
        let preview = self.current_preview.take().ok_or("no photo to accept")?;

        self.photos.push(preview);
        self.current_take += 1;
        self.retakes = 0;

        if let Some(options) = &self.options {
            let expected = options.layout.grid().total();

            if self.current_take >= expected {
                self.status = SessionStatus::Completed;
            } else {
                self.status = SessionStatus::Running;
            }
        } else {
            self.status = SessionStatus::Running;
        }

        Ok(())
    }

    pub fn is_complete(&self) -> bool {
        if let Some(options) = &self.options {
            self.current_take >= options.layout.grid().total()
        } else {
            false
        }
    }

    pub fn reset(&mut self) {
        for photo in &self.photos {
            let _ = std::fs::remove_file(photo);
        }

        if let Some(preview) = &self.current_preview {
            let _ = std::fs::remove_file(preview);
        }

        if let Some(r#final) = &self.r#final {
            let _ = std::fs::remove_file(r#final);
        }

        *self = Self::default();
    }
}

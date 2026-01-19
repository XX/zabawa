use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NotificationLevel {
    Error,
    Warning,
    Success,
    Info,
    Note,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Notification {
    pub level: NotificationLevel,
    pub message: String,
    pub creation_time: Instant,
}

impl Notification {
    pub fn new(level: NotificationLevel, message: impl Into<String>) -> Self {
        Self {
            level,
            message: message.into(),
            creation_time: Instant::now(),
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self::new(NotificationLevel::Error, message)
    }

    pub fn warning(message: impl Into<String>) -> Self {
        Self::new(NotificationLevel::Warning, message)
    }

    pub fn success(message: impl Into<String>) -> Self {
        Self::new(NotificationLevel::Success, message)
    }

    pub fn info(message: impl Into<String>) -> Self {
        Self::new(NotificationLevel::Info, message)
    }

    pub fn note(message: impl Into<String>) -> Self {
        Self::new(NotificationLevel::Note, message)
    }
}

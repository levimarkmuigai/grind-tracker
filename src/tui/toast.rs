use crate::tui::handler::ToastKind;

pub struct Toast {
    pub message: String,
    pub kind: ToastKind,
    pub expires_at: std::time::Instant,
}

impl Toast {
    pub fn new(message: impl Into<String>, kind: ToastKind, duration_sec: u64) -> Self {
        Self {
            message: message.into(),
            kind,
            expires_at: std::time::Instant::now() + std::time::Duration::from_secs(duration_sec),
        }
    }

    pub fn is_expired(&self) -> bool {
        std::time::Instant::now() >= self.expires_at
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub mod time {
    pub type Instant = std::time::Instant;
    pub type Duration = std::time::Duration;
    pub type SystemTime = std::time::SystemTime;
}

#[cfg(target_arch = "wasm32")]
pub mod time {
    pub type Instant = web_time::Instant;
    pub type SystemTime = web_time::SystemTime;
    pub type Duration = std::time::Duration;
}

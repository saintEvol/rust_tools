#[cfg(not(target_arch = "wasm32"))]
pub mod time {
    use std::time;

    pub type Instant = time::Instant;
    pub type Duration = time::Duration;
    pub type SystemTime = time::SystemTime;
    pub const UNIX_EPOCH: SystemTime = time::UNIX_EPOCH;
}

#[cfg(target_arch = "wasm32")]
pub mod time {
    pub type Instant = web_time::Instant;
    pub type SystemTime = web_time::SystemTime;
    pub type Duration = std::time::Duration;
    pub const UNIX_EPOCH: web_time::SystemTime = web_time::UNIX_EPOCH;
}

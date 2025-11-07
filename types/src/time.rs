#[cfg(not(target_arch = "wasm32"))]
pub mod time {
    pub type Instant = std::time::Instant;
    pub type Duration = std::time::Duration;
}

#[cfg(target_arch = "wasm32")]
pub mod time {
    pub type Instant = web_time::Instant;
    pub type Duration = std::time::Duration;
}

use chrono::{Duration, Local};
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct JwtPayload<PayLoadType> {
    pub token_id: String,
    pub payload: PayLoadType,
    pub expire_ms: i64,
}

impl<PayLoadType> JwtPayload<PayLoadType> {
    pub fn new(token_id: String, payload: PayLoadType, expire_in_ms: i64) -> Self {
        let iat = Local::now();
        let expire_ms = iat + Duration::milliseconds(expire_in_ms as i64);
        let expire_ms = expire_ms.timestamp_millis() as i64;
        JwtPayload {
            token_id,
            payload,
            expire_ms,
        }
    }
}

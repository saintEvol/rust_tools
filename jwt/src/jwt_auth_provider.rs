pub mod hmac_auth_provider;
pub use hmac_auth_provider::HmacAuthProvider;

/// 提供
pub trait JwtAuthProvider<Data: serde::Serialize> {
    type Error;

    /// 将载荷编码成token
    fn encode(&self, payload: &Data) -> Result<String, Self::Error>;

    /// 将token解码成载荷
    /// 注意，解码时不应该进行有效性检查，应该由调用者自行检查
    fn decode(&self, jwt: &str) -> Result<Data, Self::Error>;
}
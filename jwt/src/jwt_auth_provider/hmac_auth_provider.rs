/// 应用Hmac算法的授权器
/// 见[super::super::jwt_provider::JwtProvider]
///
use crate::jwt_auth_provider::JwtAuthProvider;
use jsonwebtoken::DecodingKey;
use jsonwebtoken::EncodingKey;
use jsonwebtoken::{Header};
use serde::de::DeserializeOwned;

pub struct HmacAuthProvider {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl HmacAuthProvider {
    /// 通过密钥进行构建
    pub fn from_secret(key: &[u8],) -> HmacAuthProvider {
        let encoding_key = EncodingKey::from_secret(key);
        let decoding_key = DecodingKey::from_secret(key);
        HmacAuthProvider {
            encoding_key,
            decoding_key,
        }
    }
}

impl<Data: serde::Serialize + DeserializeOwned> JwtAuthProvider<Data> for HmacAuthProvider {
    type Error = jsonwebtoken::errors::Error;

    fn encode(&self, payload: &Data) -> Result<String, Self::Error> {
        jsonwebtoken::encode(&Header::default(), payload, &self.encoding_key)
    }

    fn decode(&self, jwt: &str) -> Result<Data, Self::Error> {
        let mut validation = jsonwebtoken::Validation::default();
        validation.required_spec_claims.remove("exp");
        validation.validate_exp = false;

        let token_data =
            jsonwebtoken::decode::<Data>(jwt, &self.decoding_key, &validation);
        token_data.map(|token| token.claims)
    }
}

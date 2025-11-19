/// 用于用户授权的工具模块
/// 封装了token的生成，加密，存储与校验流程
/// 用户可以自定义加密与解码方式，见[super::jwt_auth_provider::JwtAuthProvider]
/// 当前提供了一些默认的加密器，见[super::jwt_auth_provider::HmacAuthProvider]
/// 用户也可以自定义生成的token的存储方式，见[super::jwt_storage_provider::JwtStorageProvider]
///
///
use crate::jwt_auth_provider::JwtAuthProvider;
use crate::jwt_payload::JwtPayload;
use crate::jwt_storage_provider::JwtStorageProvider;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AuthBody {
    pub token_id: String,
    pub token: String,
    // pub token_type: String,
    pub expire_ms: i64,
    pub expire_in_ms: i64,
}

#[derive(Debug)]
pub enum AuthError<StorageError, DecodeError> {
    StorageError(StorageError),
    DecodeError(DecodeError),
    OutOfDate,
    NoAuthDataFound,
    AuthDataNotMatch,
}

pub struct JwtProvider<JwtAuthProviderType, JwtStorageProviderType> {
    // payload: JwtPayload<JwtPayloadType>,
    expire_in_ms: i64,
    // config: JwtAuthConfig,
    auth_provider: JwtAuthProviderType,
    storage_provider: JwtStorageProviderType,
}

impl<JwtAuthProviderType, JwtStorageProviderType>
    JwtProvider<JwtAuthProviderType, JwtStorageProviderType>
where
    JwtStorageProviderType: super::jwt_storage_provider::JwtStorageProvider,
    // JwtAuthProviderType: super::jwt_auth_provider::JwtAuthProvider<JwtPayload<JwtPayloadType>>,
{
    pub fn new(
        expire_in_ms: i64,
        coder: JwtAuthProviderType,
        saver: JwtStorageProviderType,
    ) -> JwtProvider<JwtAuthProviderType, JwtStorageProviderType> {
        JwtProvider {
            // payload,
            expire_in_ms,
            auth_provider: coder,
            storage_provider: saver,
        }
    }

    /// 授权并返回token
    pub async fn authorize<JwtPayloadType>(
        &self,
        payload: JwtPayloadType,
    ) -> Result<
        AuthBody,
        AuthError<
            <JwtStorageProviderType as JwtStorageProvider>::Error,
            <JwtAuthProviderType as JwtAuthProvider<JwtPayload<JwtPayloadType>>>::Error,
        >,
    >
    where
        JwtPayloadType: serde::Serialize + std::cmp::PartialEq,
        JwtAuthProviderType: super::jwt_auth_provider::JwtAuthProvider<JwtPayload<JwtPayloadType>>,
    {
        let token_id = scru128::new_string();
        let payload = JwtPayload::new(token_id, payload, self.expire_in_ms);
        let body = self.gen_auth_body(payload);
        match body {
            Ok(body) => match self.save(body.clone()).await {
                Ok(_) => Ok(body),
                Err(e) => Err(AuthError::StorageError(e)),
            },
            Err(e) => Err(AuthError::DecodeError(e)),
        }
    }

    /// 检查指定token是否有效
    /// 从客户端收到token后，可以将token传入此函数检查其是否是一个有效的token（未过时，数据是否正确）
    pub async fn verify<JwtPayloadType>(
        &self,
        token: &str,
    ) -> Result<
        JwtPayload<JwtPayloadType>,
        AuthError<
            <JwtStorageProviderType as JwtStorageProvider>::Error,
            <JwtAuthProviderType as JwtAuthProvider<JwtPayload<JwtPayloadType>>>::Error,
        >,
    >
    where
        JwtPayloadType: serde::Serialize + std::cmp::PartialEq,
        JwtAuthProviderType: super::jwt_auth_provider::JwtAuthProvider<JwtPayload<JwtPayloadType>>,
    {
        // let auth = self.saver.load(token_id).await?;
        match self.auth_provider.decode(token) {
            Ok(payload) => match self.storage_provider.load(&payload.token_id).await {
                Ok(saved) => {
                    if let Some(saved) = saved {
                        let now = time::now_millis() as i64;
                        if now < saved.expire_ms {
                            match self.auth_provider.decode(&saved.token) {
                                Ok(saved_payload) => {
                                    if saved_payload.payload == payload.payload {
                                        Ok(saved_payload)
                                    } else {
                                        Err(AuthError::AuthDataNotMatch)
                                    }
                                }
                                Err(e) => Err(AuthError::DecodeError(e)),
                            }
                        } else {
                            Err(AuthError::OutOfDate)
                        }
                    } else {
                        Err(AuthError::NoAuthDataFound)
                    }
                }
                Err(e) => Err(AuthError::StorageError(e)),
            },
            Err(e) => Err(AuthError::DecodeError(e)),
        }
    }

    pub async fn remove<JwtPayloadType>(
        &self,
        token_id: &str,
    ) -> Result<
        Option<AuthBody>,
        <JwtStorageProviderType as JwtStorageProvider>::Error
    >
    where
        JwtPayloadType: serde::Serialize + std::cmp::PartialEq,
        JwtAuthProviderType: super::jwt_auth_provider::JwtAuthProvider<JwtPayload<JwtPayloadType>>,
    {
        let r = self.storage_provider.remove(token_id).await?;
        Ok(r)
    }

    fn gen_auth_body<JwtPayloadType>(
        &self,
        payload: JwtPayload<JwtPayloadType>,
    ) -> Result<AuthBody, <JwtAuthProviderType as JwtAuthProvider<JwtPayload<JwtPayloadType>>>::Error>
    where
        JwtPayloadType: serde::Serialize + std::cmp::PartialEq,
        JwtAuthProviderType: super::jwt_auth_provider::JwtAuthProvider<JwtPayload<JwtPayloadType>>,
    {
        let token = self.auth_provider.encode(&payload)?;
        let ab = AuthBody {
            token_id: payload.token_id.clone(),
            token,
            expire_ms: payload.expire_ms,
            expire_in_ms: self.expire_in_ms,
        };
        Ok(ab)
    }

    async fn save(
        &self,
        auth_body: AuthBody,
    ) -> Result<(), <JwtStorageProviderType as JwtStorageProvider>::Error> {
        self.storage_provider.save(auth_body).await
    }
}

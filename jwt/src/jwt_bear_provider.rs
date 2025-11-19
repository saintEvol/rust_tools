/// 专门处理http里的bear类型的jwt授权管理
/// 基本用法参考[crate::jwt::jwt_provider::JwtProvider]
///
///
use axum_core::response::{IntoResponse, Response};
use http_utils::utils::get_bear_token;
use crate::jwt_auth_provider::JwtAuthProvider;
use crate::jwt_payload::JwtPayload;
use crate::jwt_provider::{AuthBody, AuthError, JwtProvider};
use crate::jwt_storage_provider::JwtStorageProvider;
use http::request::Parts;
use http::StatusCode;

pub enum BearAuthError<StorageError, DecodeError> {
    BearError(String),
    AuthError(AuthError<StorageError, DecodeError>),
}

impl<StorageError: std::fmt::Debug, DecodeError: std::fmt::Debug> IntoResponse for BearAuthError<StorageError, DecodeError> {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            BearAuthError::BearError(e) => {
                (StatusCode::BAD_REQUEST, e)
            }
            BearAuthError::AuthError(auth_error) => {
                match auth_error {
                    AuthError::StorageError(e) => {
                        (StatusCode::INTERNAL_SERVER_ERROR, format!("{e:?}"))
                    }
                    AuthError::DecodeError(e) => {
                        (StatusCode::INTERNAL_SERVER_ERROR, format!("{e:?}"))
                    }
                    AuthError::OutOfDate => {
                        (StatusCode::UNAUTHORIZED, "token expired".to_string())
                    }
                    AuthError::NoAuthDataFound => {
                        (StatusCode::UNAUTHORIZED, "not login".to_string())
                    }
                    AuthError::AuthDataNotMatch => {
                        (StatusCode::UNAUTHORIZED, "token not match".to_string())
                    }
                }
            }
        };
        let body = axum::Json(serde_json::json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

pub struct JwtBearerProvider<JwtAuthProviderType, JwtStorageProviderType> {
    jwt_provider: JwtProvider<JwtAuthProviderType, JwtStorageProviderType>,
}

impl<JwtAuthProviderType, JwtStorageProviderType>
    JwtBearerProvider<JwtAuthProviderType, JwtStorageProviderType>
where
    JwtStorageProviderType: super::jwt_storage_provider::JwtStorageProvider,
{
    pub fn new(
        expire_in_ms: i64,
        auth_provider: JwtAuthProviderType,
        storage_provider: JwtStorageProviderType,
    ) -> Self {
        let jwt_provider = JwtProvider::new(expire_in_ms, auth_provider, storage_provider);
        JwtBearerProvider { jwt_provider }
    }

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
        self.jwt_provider.authorize(payload).await
    }

    /// 从 [http::request::Parts]中提取bear数据，并进行检验
    /// 如果检验成功，则返回token对应的payload数据
    pub async fn verify<JwtPayloadType>(
        &self,
        parts: &mut Parts,
    ) -> Result<
        JwtPayload<JwtPayloadType>,
        BearAuthError<
            <JwtStorageProviderType as JwtStorageProvider>::Error,
            <JwtAuthProviderType as JwtAuthProvider<JwtPayload<JwtPayloadType>>>::Error,
        >,
    >
    where
        JwtPayloadType: serde::Serialize + std::cmp::PartialEq,
        JwtAuthProviderType: super::jwt_auth_provider::JwtAuthProvider<JwtPayload<JwtPayloadType>>,
    {
        let token = get_bear_token(parts).await;
        match token {
            Ok(token) => {
                let ret = self.jwt_provider.verify::<JwtPayloadType>(&token).await;
                match ret {
                    Ok(ret) => Ok(ret),
                    Err(e) => Err(BearAuthError::AuthError(e)),
                }
            }
            Err(e) => Err(BearAuthError::BearError(format!("{:?}", e))),
        }
    }
}

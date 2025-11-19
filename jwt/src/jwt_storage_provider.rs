pub trait JwtStorageProvider {
    type Error;

    /// 保存授权信息
    fn save(
        &self,
        auth_body: super::jwt_provider::AuthBody,
    ) -> impl Future<Output = Result<(), Self::Error>>;

    /// 加载授权信息
    fn load(
        &self,
        token_id: &str,
    ) -> impl std::future::Future<Output = Result<Option<super::jwt_provider::AuthBody>, Self::Error>>;

    /// 删除授权信息
    /// 授权后，授权一般会失效
    fn remove(
        &self,
        token_id: &str,
    ) -> impl Future<Output = Result<Option<super::jwt_provider::AuthBody>, Self::Error>>;
}

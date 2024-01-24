use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use sea_orm::{entity::*, query::*, DatabaseConnection, EntityTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use entity::admin::{self, Entity as Admin, Model as AdminModel};

#[derive(Debug)]
pub enum AuthServiceError {
    // HashPassword,
    AccessTokenGeneration,
    RefreshTokenGeneration,
    InvalidPassword,
    UserNotFound,
    PasswordVerify,
    InvalidToken,
    TokenExpired,
    InternalError,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct JwtAccessData {
    pub id: i32,
    pub sub: String,
    pub username: String,
    pub exp: usize,
}

#[derive(Serialize, Deserialize)]
struct JwtRefreshData {
    uid: Uuid,
    exp: usize,
}

pub struct AuthService {
    db: DatabaseConnection,
}

pub trait SaltProvider {
    fn salt(&self) -> &[u8];
}

pub trait SecretsProvider {
    fn access_secret(&self) -> &[u8];
    fn refresh_secret(&self) -> &[u8];
}

impl AuthService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    // pub fn hash_password(
    //     password: &[u8],
    //     salt_provider: &impl SaltProvider,
    // ) -> Result<String, AuthServiceError> {
    //     argon2::hash_encoded(password, salt_provider.salt(), &Config::rfc9106_low_mem())
    //         .map_err(|_| AuthServiceError::HashPassword)
    // }

    pub fn validate_token(
        access_token: &str,
        secrets_provider: &impl SecretsProvider,
    ) -> Result<JwtAccessData, AuthServiceError> {
        decode::<JwtAccessData>(
            access_token,
            &DecodingKey::from_secret(secrets_provider.access_secret()),
            &Validation::default(),
        )
        .map(|jwt| jwt.claims)
        .map_err(|err| {
            log::error!("{}", err);

            match err.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthServiceError::TokenExpired,
                _ => AuthServiceError::InvalidToken,
            }
        })
    }

    pub async fn refresh_tokens(
        &self,
        user_data: &JwtAccessData,
        secrets_provider: &impl SecretsProvider,
    ) -> Result<(String, String, usize, usize), AuthServiceError> {
        let user: AdminModel = Admin::find_by_id(user_data.id)
            .one(&self.db)
            .await
            .map_err(|_| AuthServiceError::InternalError)?
            .ok_or(AuthServiceError::UserNotFound)?;

        AuthService::generate_tokens(user.id, &user.username, secrets_provider)
    }

    pub async fn authorize_user<T>(
        &self,
        username: &str,
        password: &str,
        config: &T,
    ) -> Result<(String, String, usize, usize), AuthServiceError>
    where
        T: SaltProvider + SecretsProvider,
    {
        let user: AdminModel = Admin::find()
            .filter(admin::Column::Username.eq(username))
            .one(&self.db)
            .await
            .map_err(|_| AuthServiceError::InternalError)?
            .ok_or(AuthServiceError::UserNotFound)?;

        let hashed_password = Self::verify_password(password.as_bytes(), &user.password)
            .map_err(|_| AuthServiceError::PasswordVerify)?;

        if !hashed_password {
            return Err(AuthServiceError::InvalidPassword);
        }

        Self::generate_tokens(user.id, &user.username, config).map_err(|err| match err {
            AuthServiceError::AccessTokenGeneration => AuthServiceError::AccessTokenGeneration,
            AuthServiceError::RefreshTokenGeneration => AuthServiceError::RefreshTokenGeneration,
            _ => AuthServiceError::InternalError,
        })
    }

    fn generate_tokens(
        id: i32,
        username: &str,
        secrets_provider: &impl SecretsProvider,
    ) -> Result<(String, String, usize, usize), AuthServiceError> {
        let (exp, refresh_exp) = AuthService::generate_expiration_time();
        let access_token_data = JwtAccessData {
            sub: username.to_owned(),
            id,
            username: username.to_owned(),
            exp,
        };
        let refresh_token_data = Uuid::new_v4();

        let access_token = encode(
            &Header::default(),
            &access_token_data,
            &EncodingKey::from_secret(secrets_provider.access_secret()),
        )
        .map_err(|_| AuthServiceError::AccessTokenGeneration)?;

        let refresh_token = encode(
            &Header::default(),
            &refresh_token_data,
            &EncodingKey::from_secret(secrets_provider.refresh_secret()),
        )
        .map_err(|_| AuthServiceError::RefreshTokenGeneration)?;

        Ok((access_token, refresh_token, exp, refresh_exp))
    }

    pub fn decrypt_token(
        access_token: &str,
        secrets_provider: &impl SecretsProvider,
    ) -> Result<JwtAccessData, AuthServiceError> {
        let mut validation_without_exp = Validation::default();

        validation_without_exp.validate_exp = false;

        decode::<JwtAccessData>(
            access_token,
            &DecodingKey::from_secret(secrets_provider.access_secret()),
            &validation_without_exp,
        )
        .map(|jwt| jwt.claims)
        .map_err(|err| {
            log::error!("{}", err);

            AuthServiceError::InvalidToken
        })
    }

    fn verify_password(
        input_password: &[u8],
        record_password: &str,
    ) -> Result<bool, AuthServiceError> {
        argon2::verify_encoded(record_password, input_password)
            .map_err(|_| AuthServiceError::PasswordVerify)
    }

    fn generate_expiration_time() -> (usize, usize) {
        let exp = (chrono::Utc::now() + chrono::Duration::minutes(5)).timestamp() as usize;
        let refresh_exp = (chrono::Utc::now() + chrono::Duration::days(30)).timestamp() as usize;

        (exp, refresh_exp)
    }
}

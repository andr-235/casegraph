use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use crate::errors::app_error::AppErrorDto;

pub fn hash_password(password: &str) -> Result<String, AppErrorDto> {
    let salt = SaltString::generate(&mut OsRng);

    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|err| {
            AppErrorDto::new(
                "ERR_PASSWORD_HASH",
                "Не удалось создать хэш пароля.",
                Some(err.to_string()),
            )
        })
}

#[allow(dead_code)]
pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, AppErrorDto> {
    let parsed_hash = PasswordHash::new(password_hash).map_err(|err| {
        AppErrorDto::new(
            "ERR_PASSWORD_HASH_PARSE",
            "Не удалось прочитать хэш пароля.",
            Some(err.to_string()),
        )
    })?;

    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

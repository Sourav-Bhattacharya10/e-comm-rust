use argon2::{
    Argon2,
    password_hash::{Error as ArgonError, PasswordHasher, SaltString, rand_core::OsRng},
};

pub fn hash_password(password: &str) -> Result<String, ArgonError> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    Ok(password_hash)
}

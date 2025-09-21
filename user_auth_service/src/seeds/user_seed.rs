use argon2::{
    Argon2,
    password_hash::{Error as ArgonError, PasswordHasher, SaltString, rand_core::OsRng},
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::user::User;

fn hash_password(password: &str) -> Result<String, ArgonError> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    Ok(password_hash)
}

pub async fn seeding_users_data(pg_pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    // Sample Users
    let users: Vec<User> = vec![
        User {
            id: Uuid::new_v4(),
            username: "alfa_doe".to_string(),
            email: "alfa_doe@gmail.com".to_string(),
            password_hash: "hashed_password_1".to_string(),
            role: "admin".to_string(),
            is_active: true,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        },
        User {
            id: Uuid::new_v4(),
            username: "beta_smith".to_string(),
            email: "beta_smith@gmail.com".to_string(),
            password_hash: "hashed_password_2".to_string(),
            role: "user".to_string(),
            is_active: true,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        },
    ];

    for user in users {
        let password_hash = hash_password("123").unwrap();

        sqlx::query!(
                r#"
                INSERT INTO users (id, email, username, password_hash, role, is_active, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                ON CONFLICT (email) DO NOTHING
                "#,
                user.id,
                user.email,
                user.username,
                password_hash,
                user.role,
                user.is_active,
                user.created_at,
                user.updated_at
            )
            .execute(&pg_pool)
            .await?;
    }

    println!("✅ Seeded users successfully");
    Ok(())
}

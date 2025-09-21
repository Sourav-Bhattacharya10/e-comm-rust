use async_trait::async_trait;
use sqlx::{self, PgPool};
use uuid::Uuid;

use super::repository_traits::Read;
use crate::models::user::User;

pub struct UserRepo {
    pub pool: PgPool,
}

// impl<'a> UserRepo<'a> {
// async fn create_user(
//     &self,
//     email: &str,
//     username: &str,
//     password_hash: &str,
// ) -> sqlx::Result<User> {

// }

// async fn get_all_users(&self) -> sqlx::Result<Vec<User>> {
//     let users = sqlx::query_as!(
//         User,
//         r#"
//         SELECT id, username, email, password_hash, role, is_active, created_at, updated_at
//         FROM users
//         "#
//     )
//     .fetch_all(self.pool)
//     .await?;

//     Ok(users)
// }
// }

#[async_trait]
impl Read<User, Uuid> for UserRepo {
    // async fn create(&self, entity: Self::Entity) -> Result<Self::Entity, Box<dyn std::error::Error>> {
    //     let rec = sqlx::query_as!(
    //         User,
    //         r#"
    //         INSERT INTO users (id, username, email, password_hash, role, is_active, created_at, updated_at)
    //         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
    //         RETURNING id, username, email, password_hash, role, is_active, created_at, updated_at
    //         "#,
    //         entity.id,
    //         entity.username,
    //         entity.email,
    //         entity.password_hash,
    //         entity.role,
    //         entity.is_active,
    //         entity.created_at,
    //         entity.updated_at
    //     )
    //     .fetch_one(self.pool)
    //     .await?;

    //     Ok(rec)
    // }

    async fn read(&self, id: Uuid) -> Result<Option<User>, Box<dyn std::error::Error>> {
        let rec = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, role, is_active, created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(rec)
    }

    async fn read_all(&self) -> Result<Vec<User>, Box<dyn std::error::Error>> {
        let recs = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, role, is_active, created_at, updated_at
            FROM users
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(recs)
    }

    // async fn update(
    //     &self,
    //     id: Self::Id,
    //     entity: Self::Entity,
    // ) -> Result<Self::Entity, Box<dyn std::error::Error>> {
    //     let rec = sqlx::query_as!(
    //         User,
    //         r#"
    //         UPDATE users
    //         SET username = $2,
    //             email = $3,
    //             password_hash = $4,
    //             role = $5,
    //             is_active = $6,
    //             updated_at = $7
    //         WHERE id = $1
    //         RETURNING id, username, email, password_hash, role, is_active, created_at, updated_at
    //         "#,
    //         id,
    //         entity.username,
    //         entity.email,
    //         entity.password_hash,
    //         entity.role,
    //         entity.is_active,
    //         entity.updated_at
    //     )
    //     .fetch_one(self.pool)
    //     .await?;
    // }
}

use async_trait::async_trait;
use sqlx::{self, PgPool};
use std::error::Error;
use uuid::Uuid;

use super::repository_traits::{Create, Delete, Read, Update};
use crate::models::user::User;

pub struct UserRepo {
    pub pool: PgPool,
}

#[async_trait]
impl Read<User, Uuid> for UserRepo {
    async fn read(&self, id: Uuid) -> Result<Option<User>, Box<dyn Error>> {
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

    async fn read_all(&self) -> Result<Vec<User>, Box<dyn Error>> {
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
}

#[async_trait]
impl Create<User> for UserRepo {
    async fn create(&self, entity: User) -> Result<User, Box<dyn Error>> {
        let rec = sqlx::query_as!(
                    User,
                    r#"
                    INSERT INTO users (id, username, email, password_hash, role, is_active, created_at, updated_at)
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                    RETURNING id, username, email, password_hash, role, is_active, created_at, updated_at
                    "#,
                    entity.id,
                    entity.username,
                    entity.email,
                    entity.password_hash,
                    entity.role,
                    entity.is_active,
                    entity.created_at,
                    entity.updated_at
                )
                .fetch_one(&self.pool)
                .await?;

        Ok(rec)
    }
}

#[async_trait]
impl Update<User, Uuid> for UserRepo {
    async fn update(&self, id: Uuid, entity: User) -> Result<User, Box<dyn Error>> {
        let rec = sqlx::query_as!(
            User,
            r#"
            UPDATE users
                SET username = $2,
                    email = $3,
                    role = $4,
                    is_active = $5,
                    updated_at = $6
            WHERE id = $1
            RETURNING id, username, email, password_hash, role, is_active, created_at, updated_at
            "#,
            id,
            entity.username,
            entity.email,
            entity.role,
            entity.is_active,
            entity.updated_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(rec)
    }

    async fn update_is_active(&self, id: Uuid, is_active: bool) -> Result<User, Box<dyn Error>> {
        let rec = sqlx::query_as!(
            User,
            r#"
            UPDATE users
                SET is_active = $2,
                    updated_at = $3
            WHERE id = $1
            RETURNING id, username, email, password_hash, role, is_active, created_at, updated_at
            "#,
            id,
            is_active,
            Some(chrono::Utc::now())
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(rec)
    }
}

#[async_trait]
impl Delete<User, Uuid> for UserRepo {
    async fn delete(&self, id: Uuid) -> Result<User, Box<dyn Error>> {
        let rec = sqlx::query_as!(
            User,
            r#"
            DELETE FROM users
            WHERE id = $1
            RETURNING id, username, email, password_hash, role, is_active, created_at, updated_at
            "#,
            id,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(rec)
    }
}

use async_trait::async_trait;
use sqlx::{self, PgPool, QueryBuilder};
use std::error::Error;
use uuid::Uuid;

use super::repository_traits::{Create, Delete, Read, Update};
use crate::models::user::User;

pub struct UserRepo {
    pub pool: PgPool,
}

#[async_trait]
impl Read<User, Uuid> for UserRepo {
    async fn read(&self, id: Uuid) -> Result<Option<User>, Box<dyn Error + Send + Sync>> {
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

    async fn read_all(
        &self,
        name: Option<String>,
        limit: u32,
        offset: u32,
        order_by: &str,
    ) -> Result<Vec<User>, Box<dyn Error + Send + Sync>> {
        let allowed_columns = [
            "id",
            "username",
            "email",
            "role",
            "is_active",
            "created_at",
            "updated_at",
        ];
        let order_by_clause = if allowed_columns.contains(&order_by) {
            order_by
        } else {
            "id" // default to id
        };

        let mut query_builder: QueryBuilder<sqlx::Postgres> = QueryBuilder::new(
            "SELECT id, username, email, password_hash, role, is_active, created_at, updated_at FROM users",
        );

        if let Some(name_word) = name {
            query_builder.push(" WHERE username LIKE ");
            query_builder.push_bind(format!("%{}%", name_word));
        }

        query_builder.push(format!(" ORDER BY {} ", order_by_clause));

        query_builder.push(" LIMIT ");
        query_builder.push_bind(limit as i64);

        query_builder.push(" OFFSET ");
        query_builder.push_bind(offset as i64);

        let query = query_builder.build_query_as();
        let recs = query.fetch_all(&self.pool).await?;

        Ok(recs)
    }

    async fn count_total(&self) -> Result<u64, Box<dyn Error + Send + Sync>> {
        let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
            .fetch_one(&self.pool)
            .await?;

        Ok(total as u64)
    }
}

#[async_trait]
impl Create<User> for UserRepo {
    async fn create(&self, entity: User) -> Result<User, Box<dyn Error + Send + Sync>> {
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
    async fn update(&self, id: Uuid, entity: User) -> Result<User, Box<dyn Error + Send + Sync>> {
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

    async fn update_is_active(
        &self,
        id: Uuid,
        is_active: bool,
    ) -> Result<User, Box<dyn Error + Send + Sync>> {
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
    async fn delete(&self, id: Uuid) -> Result<User, Box<dyn Error + Send + Sync>> {
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

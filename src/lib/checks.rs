use crate::types::{Context, Error};
use poise::serenity_prelude::User;
use sqlx::{Pool, Postgres};
use tracing::{event, Level};
use tracing_unwrap::ResultExt;

use crate::structs::UserRow;

/// This code adds a user to the `users` table in the database if they are not already in the table.
///
/// Note that the `users` table has a column named `id` which is of type `text` and not `bigint`.
pub async fn user_db_check(db: Pool<Postgres>, user: User) {
    // Check if user already exists in database
    let optional_user: Option<UserRow> =
        sqlx::query_as("SELECT * FROM users WHERE (id) = ($1) LIMIT 1;")
            .bind(user.id.0.to_string())
            .fetch_optional(&db)
            .await
            .expect_or_log("Failed to select user from db");

    // If user does not exist, create a new row in the database
    if let Some(_) = optional_user {
    } else {
        let _row: UserRow = sqlx::query_as("INSERT INTO users (id) VALUES ($1) RETURNING *;")
            .bind(&user.id.0.to_string())
            .fetch_one(&db)
            .await
            .expect_or_log("Failed to add user to db");

        let user_info = format!("ID: {} || Current Useranme: {}", user.id.0, user.name);

        event!(Level::INFO, "Added new user to `users` db." = user_info);
    }
}

// is_admin checks if a user is in the `users` table and than if they have the `is_admin` column set to `true`.
// if the user is the bot owner, they are automatically considered an admin.
// return true if the user is an admin or the bot owner, false if they are not.
pub async fn is_admin(ctx: Context<'_>) -> Result<bool, Error> {
    let owners = ctx.framework().options().owners.clone();

    if owners.contains(&ctx.author().id) {
        event!(Level::INFO, "Admin has run an elevated command." = &ctx.author().id.0);
        return Ok(true);
    }

    let optional_user: Option<UserRow> =
        sqlx::query_as("SELECT * FROM users WHERE (id) = ($1) LIMIT 1;")
            .bind(ctx.author().id.0.to_string())
            .fetch_optional(&ctx.data().db.clone())
            .await
            .expect_or_log("Failed to select user from db");

    if let Some(user) = optional_user {
        if user.is_admin {
            event!(Level::INFO, "Admin has run an elevated command." = user.id);
            return Ok(true);
        } else {
            return Ok(false);
        }
    } else {
        return Ok(false);
    }
}

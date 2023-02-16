use poise::serenity_prelude::User;
use sqlx::{Pool, Postgres};
use tracing::{event, instrument, Level};
use tracing_unwrap::ResultExt;

use crate::structs::UserRow;

pub async fn user_db_check(db: Pool<Postgres>, user: User) {
    let optional_user: Option<UserRow> =
        sqlx::query_as("SELECT * FROM users WHERE (id) = ($1) LIMIT 1;")
            .bind(user.id.0.to_string())
            .fetch_optional(&db)
            .await
            .expect_or_log("Failed to select user from db");

    match optional_user {
        Some(_) => {}
        None => {
            let _row: UserRow = sqlx::query_as("INSERT INTO users (id) VALUES ($1) RETURNING *;")
                .bind(&user.id.0.to_string())
                .fetch_one(&db)
                .await
                .expect_or_log("Failed to add user to db");

            let user_info = format!("ID: {} || Current Useranme: {}", user.id.0, user.name);

            event!(Level::INFO, "Added new user to `users` db." = user_info);
        }
    }
}

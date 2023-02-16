use rusted_wumpus_lib::structs::QuoteRow;
use tracing::instrument;

use crate::{Context, Error};

/// Gets a quote by ID
#[poise::command(prefix_command, slash_command, category = "Quotes")]
pub async fn getquote(
    ctx: Context<'_>,
    #[description = "ID"] quote_id: String,
) -> Result<(), Error> {
    let pool = ctx.data().db.clone();

    let row: QuoteRow = sqlx::query_as("SELECT quote FROM quotes WHERE id = $1")
        .bind(&quote_id.trim())
        .fetch_one(&pool)
        .await?;

    ctx.say(format!(
        "Quote {}: {}\nAdded by: {}",
        row.id, row.quote, row.author
    ))
    .await?;

    Ok(())
}

/// Gets a random quote
#[poise::command(prefix_command, slash_command, category = "Quotes")]
pub async fn randquote(ctx: Context<'_>) -> Result<(), Error> {
    let pool = ctx.data().db.clone();

    let quote: QuoteRow = sqlx::query_as("SELECT * FROM quotes ORDER BY random() LIMIT 1;")
        .fetch_one(&pool)
        .await?;

    ctx.say(format!(
        "Quote {}: {}\n Added by: <@{}>",
        quote.id, quote.quote, quote.author
    ))
    .await?;

    Ok(())
}

/// Add a new quote
#[poise::command(prefix_command, slash_command, category = "Quotes")]
pub async fn addquote(ctx: Context<'_>, #[description = "ID"] quote: String) -> Result<(), Error> {
    ctx.defer().await?;

    let pool = ctx.data().db.clone();

    let row: QuoteRow = sqlx::query_as("INSERT INTO quotes (quote, author) VALUES ($1, $2) RETURNING *")
        .bind(&quote.trim())
        .bind(ctx.author().id.0.to_string())
        .fetch_one(&pool)
        .await?;

    ctx.say(format!("Added quote {}: {}", row.id, row.quote))
        .await?;

    Ok(())
}

use poise::serenity_prelude::UserId;
use rusted_wumpus_lib::structs::QuoteRow;
use tracing::{event, instrument, Level};

use crate::{Context, Error};

/// Gets a quote by ID
#[instrument]
#[poise::command(prefix_command, slash_command, category = "Quotes")]
pub async fn getquote(
    ctx: Context<'_>,
    #[description = "Quote ID"] quote_id: String,
) -> Result<(), Error> {
    let row: Option<QuoteRow> = sqlx::query_as("SELECT * FROM quotes WHERE (id) = ($1) LIMIT 1;")
        .bind(quote_id.trim())
        .fetch_optional(&ctx.data().db.clone())
        .await?;

    if let Some(q) = row {
        let author_id = UserId::from(q.author.parse::<u64>()?);
        let author = author_id.to_user(ctx).await?;

        ctx.say(format!(
            "Quote {}: {}\n Added by: {}",
            q.id, q.quote, author.name
        ))
        .await?;
    } else {
        ctx.say(format!("Quote {} not found", quote_id)).await?;
    }

    Ok(())
}

/// Gets a random quote
#[instrument]
#[poise::command(prefix_command, slash_command, category = "Quotes")]
pub async fn randquote(ctx: Context<'_>) -> Result<(), Error> {
    let pool = ctx.data().db.clone();

    let quote: Option<QuoteRow> = sqlx::query_as("SELECT * FROM quotes ORDER BY random() LIMIT 1;")
        .fetch_optional(&pool)
        .await?;

    let quote = match quote {
        Some(quote_row) => quote_row,
        None => {
            ctx.say("No quotes found").await?;
            return Ok(());
        }
    };

    let author_id = UserId::from(quote.author.parse::<u64>().unwrap());
    let author = author_id.to_user(ctx).await?;
    ctx.say(format!(
        "Quote {}: {}\n Added by: {}",
        quote.id, quote.quote, author.name
    ))
    .await?;

    Ok(())
}

/// Add a new quote
#[instrument]
#[poise::command(prefix_command, slash_command, category = "Quotes")]
pub async fn addquote(
    ctx: Context<'_>,
    #[description = "Quote contents"] quote: String,
) -> Result<(), Error> {
    // Prepare the database connection for the query.
    let pool = ctx.data().db.clone();

    // Create a new quote in the database and return the created row.
    let row: QuoteRow =
        sqlx::query_as("INSERT INTO quotes (quote, author) VALUES ($1, $2) RETURNING *")
            .bind(quote.trim())
            .bind(ctx.author().id.0.to_string())
            .fetch_one(&pool)
            .await?;

    // Send a message saying the quote was added.
    ctx.say(format!("Added quote {}: {}", row.id, row.quote))
        .await?;

    Ok(())
}

/// Delete a quote via ID
#[instrument]
#[poise::command(prefix_command, slash_command, category = "Quotes")]
pub async fn delquote(
    ctx: Context<'_>,
    #[description = "Quote ID"] quote_id: String,
) -> Result<(), Error> {
    // Prepare the database connection for the query.
    let pool = ctx.data().db.clone();

    // Delete the quote from the database.
    let removed_row: QuoteRow = sqlx::query_as("DELETE FROM quotes WHERE (id) = ($1) RETURNING *;")
        .bind(quote_id.trim())
        .fetch_optional(&pool)
        .await
        .unwrap()
        .expect("Failed to remove quote, probably because the quote with that ID doesn't exist");

    // Send a message saying the quote was removed.
    ctx.say(format!(
        "Removed quote {}\nContents: {}",
        removed_row.id, removed_row.quote
    ))
    .await?;

    Ok(())
}

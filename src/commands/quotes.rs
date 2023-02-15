use crate::{Context, Error};

/// Gets a quote by ID
#[poise::command(prefix_command, slash_command, category = "Quotes")]
pub async fn getquote(ctx: Context<'_>, #[description = "ID"] quote_id: String) -> Result<(), Error> {
    let pool = ctx.data().db.clone();

    let row: (String,) = sqlx::query_as("SELECT quote FROM quotes WHERE id = $1")
        .bind(&quote_id.trim())
        .fetch_one(&pool)
        .await?;

    ctx.say(format!("Quote {}: {}", &quote_id.trim(), row.0)).await?;

    Ok(())
}

/// Gets a random quote
#[poise::command(prefix_command, slash_command, category = "Quotes")]
pub async fn randquote(ctx: Context<'_>) -> Result<(), Error> {
    let pool = ctx.data().db.clone();

        #[derive(Debug)]
        struct Quote {
            id: String,
            quote: String,
            creator: String,
        }

    let quote = sqlx::query_as!(Quote, "SELECT * FROM quotes ORDER BY random() LIMIT 1;")
        .fetch_one(&pool)
        .await?;

    ctx.say(format!("Quote {}: {}\n Added by: <@{}>", quote.id, quote.quote, quote.creator)).await?;

    Ok(())
}

/// Add a new quote
#[poise::command(prefix_command, slash_command, category = "Quotes")]
pub async fn addquote(ctx: Context<'_>, #[description = "ID"] quote: String) -> Result<(), Error> {
    let pool = ctx.data().db.clone();

    sqlx::query!("INSERT INTO quotes (quote, creator) VALUES ($1, $2)", &quote, ctx.author().id.0.to_string()).execute(&pool).await?;

    #[derive(Debug)]
    struct QuoteWithID {
        id: String,
        quote: String
    }

    let retuned_quote =
        sqlx::query_as!(QuoteWithID, r#"SELECT id, quote FROM quotes WHERE quote = $1"#, &quote)
            .fetch_one(&pool)
            .await?;

    ctx.say(format!(
        "Added quote {}: {}",
        retuned_quote.id, retuned_quote.quote
    ))
    .await?;

    Ok(())
}
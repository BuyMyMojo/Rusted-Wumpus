use crate::{ Context, Error };
use entity::quote;
use entity::quote::Entity as Quote;
use migration::{ Query, Func, Order, FunctionCall, Function };
use sea_orm::{ EntityTrait, Set };

/// Gets a quote by ID
#[poise::command(prefix_command, slash_command, category = "Quotes")]
pub async fn getquote(
    ctx: Context<'_>,
    #[description = "ID"] quote_id: String
) -> Result<(), Error> {
    let result: Option<quote::Model> = Quote::find_by_id(&quote_id).one(&ctx.data().db).await?;

    match result {
        None => {
            ctx.say(format!("Unable to find quote with the ID {}", &quote_id)).await?;
        }
        Some(q) => {
            ctx.say(format!("Quote {}: {}\nAdded by: {}", q.id, q.quote, q.author)).await?;
        }
    }

    Ok(())
}

/// Add a new quote
#[poise::command(prefix_command, slash_command, category = "Quotes")]
pub async fn addquote(
    ctx: Context<'_>,
    #[description = "ID"] quote_contents: String
) -> Result<(), Error> {
    let new_quote = quote::ActiveModel {
        quote: Set(quote_contents),
        author: Set(format!("{}", ctx.author().id.0)),
        ..Default::default() // all other attributes are `NotSet`
    };

    let insert_result = Quote::insert(new_quote).exec(&ctx.data().db).await?;

    let select_result: Option<quote::Model> = Quote::find_by_id(
        insert_result.last_insert_id.to_string()
    ).one(&ctx.data().db).await?;

    let final_quote = select_result.unwrap();

    ctx.say(format!("Added quote {}: {}", final_quote.id, final_quote.quote)).await?;

    Ok(())
}
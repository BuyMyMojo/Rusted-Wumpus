use chrono::NaiveDateTime;

use poise::serenity_prelude::{ AttachmentType, Colour };

use reqwest::Client;
use sea_orm::{ConnectOptions, Database};
use migration::{Migrator, MigratorTrait};
use serde_json::json;

use html2text::from_read;
use dotenv::dotenv;
use std::time::{Instant, Duration};

use std::{ sync::mpsc, thread }; // Multithreading // Time tracking

use owoify::OwOifiable;

use poise::serenity_prelude as serenity;

use std::vec;

use clap::Parser;

// Variables stores more cleanly
mod vars;
use vars::ANIME_QUERY;
use vars::HELP_EXTRA_TEXT;
use vars::INFO_MESSAGE;
use vars::MANGA_QUERY;

mod commands;
use commands::quotes;

#[derive(Debug)]
pub struct Data {
    pub db: sea_orm::DatabaseConnection,
}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Address of Redis server
    #[clap(
        short,
        long,
        env = "DATABASE_URL",
        default_value = "postgres://postgres:postgres@localhost/postgres"
    )]
    database_url: String,

    /// Discord bot token
    #[clap(short, long, env = "BOT_TOKEN", default_value = "")]
    token: String,
}

/// Show this help menu
#[poise::command(prefix_command, track_edits, slash_command, category = "Info")]
async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"] #[autocomplete = "poise::builtins::autocomplete_command"] command: Option<String>
) -> Result<(), Error> {
    poise::builtins::help(ctx, command.as_deref(), poise::builtins::HelpConfiguration {
        extra_text_at_bottom: HELP_EXTRA_TEXT,
        show_context_menu_commands: true,
        ..Default::default()
    }).await?;
    Ok(())
}

// Create commands bellow!

/// Display your or another user's account creation date
#[poise::command(prefix_command, slash_command, track_edits)]
pub async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>
) -> Result<(), Error> {
    let user = user.as_ref().unwrap_or(ctx.author());
    ctx.say(format!("{}'s account was created at {}", user.name, user.created_at())).await?;

    Ok(())
}

/// Register application commands in this guild or globally
///
/// Run with no arguments to register in guild, run with argument "global" to register globally.
#[poise::command(prefix_command, slash_command, hide_in_help, owners_only)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::samples::register_application_commands_buttons(ctx).await?;

    Ok(())
}

/// Replies with pong!
#[poise::command(prefix_command, slash_command, category = "Miscellaneous")]
async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Pong!").await?;

    Ok(())
}

/// Replies with pog pog pog!
#[poise::command(prefix_command, slash_command, category = "Fun")]
async fn pog(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("pog pog pog!").await?;

    Ok(())
}

/// Replies with some basic info
#[poise::command(prefix_command, slash_command, category = "Info")]
async fn info(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say(INFO_MESSAGE).await?;

    Ok(())
}

/// OwOifys your message
#[poise::command(prefix_command, slash_command, category = "Fun")]
async fn owo(ctx: Context<'_>, #[description = "Message"] msg: String) -> Result<(), Error> {
    ctx.say(String::from(msg).owoify()).await?;

    Ok(())
}

/// Get an AniList entry for an Anime
#[poise::command(prefix_command, slash_command, category = "Fun")]
async fn anime(
    ctx: Context<'_>,
    #[description = "Name"] msg: String,
    #[description = "Output raw json"] raw: Option<bool>
) -> Result<(), Error> {
    // Tell discord wait longer then 3 seconds
    ctx.defer().await?;

    let client = Client::new();

    // Define query and variables
    let json = json!({"query": ANIME_QUERY, "variables": {"search": format!("{}", msg)}});

    // Make HTTP post request
    let resp = client
        .post("https://graphql.anilist.co/")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .body(json.to_string())
        .send().await
        .unwrap()
        .text().await;

    // Get json
    let result: serde_json::Value = serde_json::from_str(&resp.unwrap()).unwrap();

    let formatted_json = format!("{:#?}", result);

    // let anime_id = result["data"]["Media"]["id"].as_u64().unwrap();
    let description = from_read(
        result["data"]["Media"]["description"].as_str().unwrap().as_bytes(),
        50
    );
    let status = result["data"]["Media"]["status"].as_str().unwrap();
    let anilist_url = result["data"]["Media"]["siteUrl"].as_str().unwrap();
    let episode_count = result["data"]["Media"]["episodes"].as_u64().unwrap();
    let average_episode_length = result["data"]["Media"]["duration"].as_u64().unwrap();
    let average_score = result["data"]["Media"]["averageScore"].as_u64().unwrap();
    let median_score = result["data"]["Media"]["meanScore"].as_u64().unwrap();
    let adult = result["data"]["Media"]["isAdult"].as_bool().unwrap();

    let romaji_title = result["data"]["Media"]["title"]["romaji"].as_str().unwrap();
    let mut english_title = result["data"]["Media"]["title"]["romaji"].as_str().unwrap();
    if result["data"]["Media"]["title"]["english"].as_str() != None {
        english_title = result["data"]["Media"]["title"]["english"].as_str().unwrap();
    }

    let mut base_colour = "#aed6f1";
    if result["data"]["Media"]["coverImage"]["color"].as_str() != None {
        base_colour = result["data"]["Media"]["coverImage"]["color"].as_str().unwrap();
    }

    let image = result["data"]["Media"]["coverImage"]["extraLarge"].as_str().unwrap();
    let small_image = result["data"]["Media"]["coverImage"]["large"].as_str().unwrap();

    let mut season = "N/A";
    if result["data"]["Media"]["season"].as_str() != None {
        season = result["data"]["Media"]["season"].as_str().unwrap();
    }

    let mut start_year: i64 = -1;
    if result["data"]["Media"]["startDate"]["year"].as_i64() != None {
        start_year = result["data"]["Media"]["startDate"]["year"].as_i64().unwrap();
    }
    let mut start_month: i64 = -1;
    if result["data"]["Media"]["startDate"]["month"].as_i64() != None {
        start_month = result["data"]["Media"]["startDate"]["month"].as_i64().unwrap();
    }
    let mut start_day: i64 = -1;
    if result["data"]["Media"]["startDate"]["day"].as_i64() != None {
        start_day = result["data"]["Media"]["startDate"]["day"].as_i64().unwrap();
    }

    let mut end_year: i64 = -1;
    if result["data"]["Media"]["endDate"]["year"].as_i64() != None {
        end_year = result["data"]["Media"]["endDate"]["year"].as_i64().unwrap();
    }
    let mut end_month: i64 = -1;
    if result["data"]["Media"]["endDate"]["month"].as_i64() != None {
        end_month = result["data"]["Media"]["endDate"]["month"].as_i64().unwrap();
    }
    let mut end_day: i64 = -1;
    if result["data"]["Media"]["endDate"]["day"].as_i64() != None {
        end_day = result["data"]["Media"]["endDate"]["day"].as_i64().unwrap();
    }

    let without_prefix = base_colour.trim_start_matches("#");
    let colour_i32 = i32::from_str_radix(without_prefix, 16).unwrap();

    let field_list = [
        ("English Name", format!("{}", english_title), true),
        ("Romaji Name", format!("{}", romaji_title), true),
        ("Description", format!("{}", description), false),
        ("Start Date", format!("{} {}/{}/{}", season, start_year, start_month, start_day), true),
        ("End Date", format!("{}/{}/{}", end_year, end_month, end_day), true),
        ("Status", format!("{}", status), true),
        ("Episode Count", format!("{}", episode_count), true),
        ("Episode Length", format!("{} minutes", average_episode_length), true),
        ("Average score", format!("{}", average_score), true),
        ("Mean score", format!("{}", median_score), true),
        ("Is adult?", format!("{}", adult), true),
    ];

    if raw != None {
        if raw.unwrap() == true {
            ctx.send(|f| {
                f.content("Anime result")
                    .ephemeral(false)
                    .attachment(AttachmentType::Bytes {
                        data: std::borrow::Cow::Borrowed(formatted_json.as_bytes()),
                        filename: String::from("Anime.json"),
                    })
            }).await?;
        } else {
            ctx.send(|f| {
                f.embed(|b| {
                    b.colour(Colour::from(colour_i32).tuple())
                        .description("Anime Result")
                        .image(image)
                        .author(|f| f.icon_url(small_image).name("AniList").url(anilist_url))
                        .fields(field_list)
                })
            }).await?;
        }
    } else {
        ctx.send(|f| {
            f.embed(|b| {
                b.colour(Colour::from(colour_i32).tuple())
                    .description("Anime Result")
                    .image(image)
                    .author(|f| f.icon_url(small_image).name("AniList").url(anilist_url))
                    .fields(field_list)
            })
        }).await?;
    }
    Ok(())
}

/// Get an AniList entry for a Manga
#[poise::command(prefix_command, slash_command, category = "Fun")]
async fn manga(
    ctx: Context<'_>,
    #[description = "Name"] msg: String,
    #[description = "Output raw json"] raw: Option<bool>
) -> Result<(), Error> {
    // Tell discord wait longer then 3 seconds
    ctx.defer().await?;

    let client = Client::new();

    // Define query and variables
    let json = json!({"query": MANGA_QUERY, "variables": {"search": format!("{}", msg)}});

    // Make HTTP post request
    let resp = client
        .post("https://graphql.anilist.co/")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .body(json.to_string())
        .send().await
        .unwrap()
        .text().await;

    // Get json
    let result: serde_json::Value = serde_json::from_str(&resp.unwrap()).unwrap();

    let formatted_json = format!("{:#?}", result);

    if raw != None {
        if raw.unwrap() == true {
            ctx.send(|f| {
                f.content("Anime result")
                    .ephemeral(false)
                    .attachment(AttachmentType::Bytes {
                        data: std::borrow::Cow::Borrowed(formatted_json.as_bytes()),
                        filename: String::from("Anime.json"),
                    })
            }).await?;

            return Ok(());
        }
    }

    // let anime_id = result["data"]["Media"]["id"].as_u64().unwrap();
    let description = from_read(
        result["data"]["Media"]["description"].as_str().unwrap().as_bytes(),
        50
    );
    let status = result["data"]["Media"]["status"].as_str().unwrap();
    let anilist_url = result["data"]["Media"]["siteUrl"].as_str().unwrap();
    let mut volume_count: i64 = -1;
    if result["data"]["Media"]["volumes"].as_i64() != None {
        volume_count = result["data"]["Media"]["volumes"].as_i64().unwrap();
    }
    let chapter_coumt = result["data"]["Media"]["chapters"].as_u64().unwrap();
    let average_score = result["data"]["Media"]["averageScore"].as_u64().unwrap();
    let median_score = result["data"]["Media"]["meanScore"].as_u64().unwrap();
    let adult = result["data"]["Media"]["isAdult"].as_bool().unwrap();

    let romaji_title = result["data"]["Media"]["title"]["romaji"].as_str().unwrap();
    let mut english_title = result["data"]["Media"]["title"]["romaji"].as_str().unwrap();
    if result["data"]["Media"]["title"]["english"].as_str() != None {
        english_title = result["data"]["Media"]["title"]["english"].as_str().unwrap();
    }

    let mut base_colour = "#aed6f1";
    if result["data"]["Media"]["coverImage"]["color"].as_str() != None {
        base_colour = result["data"]["Media"]["coverImage"]["color"].as_str().unwrap();
    }

    let image = result["data"]["Media"]["coverImage"]["extraLarge"].as_str().unwrap();
    let small_image = result["data"]["Media"]["coverImage"]["large"].as_str().unwrap();

    let mut season = "N/A";
    if result["data"]["Media"]["season"].as_str() != None {
        season = result["data"]["Media"]["season"].as_str().unwrap();
    }

    let mut start_year: i64 = -1;
    if result["data"]["Media"]["startDate"]["year"].as_i64() != None {
        start_year = result["data"]["Media"]["startDate"]["year"].as_i64().unwrap();
    }
    let mut start_month: i64 = -1;
    if result["data"]["Media"]["startDate"]["month"].as_i64() != None {
        start_month = result["data"]["Media"]["startDate"]["month"].as_i64().unwrap();
    }
    let mut start_day: i64 = -1;
    if result["data"]["Media"]["startDate"]["day"].as_i64() != None {
        start_day = result["data"]["Media"]["startDate"]["day"].as_i64().unwrap();
    }

    let mut end_year: i64 = -1;
    if result["data"]["Media"]["endDate"]["year"].as_i64() != None {
        end_year = result["data"]["Media"]["endDate"]["year"].as_i64().unwrap();
    }
    let mut end_month: i64 = -1;
    if result["data"]["Media"]["endDate"]["month"].as_i64() != None {
        end_month = result["data"]["Media"]["endDate"]["month"].as_i64().unwrap();
    }
    let mut end_day: i64 = -1;
    if result["data"]["Media"]["endDate"]["day"].as_i64() != None {
        end_day = result["data"]["Media"]["endDate"]["day"].as_i64().unwrap();
    }

    let without_prefix = base_colour.trim_start_matches("#");
    let colour_i32 = i32::from_str_radix(without_prefix, 16).unwrap();

    let field_list = [
        ("English Name", format!("{}", english_title), true),
        ("Romaji Name", format!("{}", romaji_title), true),
        ("Description", format!("{}", description), false),
        ("Start Date", format!("{} {}/{}/{}", season, start_year, start_month, start_day), true),
        ("End Date", format!("{}/{}/{}", end_year, end_month, end_day), true),
        ("Status", format!("{}", status), true),
        ("Volume Count", format!("{}", volume_count), true),
        ("Chapter Count", format!("{} minutes", chapter_coumt), true),
        ("Average Score", format!("{}", average_score), true),
        ("Mean Score", format!("{}", median_score), true),
        ("Is Adult?", format!("{}", adult), true),
    ];

    ctx.send(|f| {
        f.embed(|b| {
            b.colour(Colour::from(colour_i32).tuple())
                .description("Anime Result")
                .image(image)
                .author(|f| f.icon_url(small_image).name("AniList").url(anilist_url))
                .fields(field_list)
        })
    }).await?;
    Ok(())
}

/// Tests multithreaded functionality. use -t to show how long the threads live for
#[poise::command(prefix_command, slash_command, category = "Testing")]
#[cfg(feature = "testing")]
async fn threadtest(ctx: Context<'_>, #[description = "Timed"] timed: bool) -> Result<(), Error> {
    // Main math channels
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    // Optional duration channels
    let (tx3, rx3) = mpsc::channel();
    let (tx4, rx4) = mpsc::channel();

    thread::spawn(move || {
        let start = Instant::now(); // Start time tracking

        let channel_msg = 69 + 420; // Math
        tx1.send(channel_msg).unwrap(); // Send math over channel 1
        println!("Sent {} on channel 1!", channel_msg); // Print once channel 1 takes the message

        let duration = (start.elapsed().as_nanos() as f64) / (1000000 as f64); // End time tracking
        tx3.send(duration).unwrap(); // Send the ms taken
    });

    thread::spawn(move || {
        let start = Instant::now();
        let channel_msg = 420 * 2;
        tx2.send(channel_msg).unwrap();
        println!("Sent {} on channel 2!", channel_msg);
        let duration = (start.elapsed().as_nanos() as f64) / (1000000 as f64);
        tx4.send(duration).unwrap();
    });

    ctx.say(
        format!(
            "Thread 1 returned: {}\nThread 2 returned: {}",
            rx1.recv().unwrap(),
            rx2.recv().unwrap()
        )
    ).await?; // This line wont actually complete until both threads are firing in their channels

    if
        timed
        // <>threadtest -t
    {
        ctx.say(
            format!(
                "Thread 1 took {}ms to complete\nThread 2 took {}ms to complete",
                rx3.recv().unwrap(),
                rx4.recv().unwrap()
            )
        ).await?;
    } else {
        // I'm just throwing away these channels unless being called since this is a test command. probably wouldn't leave the time tracking in at all if this was a more functional command
        let _ = rx3.recv().unwrap();
        let _ = rx4.recv().unwrap();
    }

    Ok(())
}

/// Gets the creation date or a Snowflake ID
#[poise::command(prefix_command, slash_command, category = "Tools")]
async fn creationdate(
    ctx: Context<'_>,
    #[description = "ID"] snowflake_id: u128
) -> Result<(), Error> {
    let unix_timecode = snowflake_to_unix(snowflake_id);

    let date_time_stamp = NaiveDateTime::from_timestamp_opt(unix_timecode as i64, 0);

    if date_time_stamp.is_none() {
        ctx.say("Unable to retrieve timestamp from snowflake").await?;
    } else {
        ctx.say(format!("Created/Joined on {}", date_time_stamp.unwrap())).await?;
    }

    Ok(())
}

// Place other functions bellow here

/// Converts a dsicord snowflake to a unix timecode
fn snowflake_to_unix(id: u128) -> u128 {
    const DISCORD_EPOCH: u128 = 1420070400000;

    let unix_timecode = ((id >> 22) + DISCORD_EPOCH) / 1000;

    return unix_timecode;
}

// TODO: Add quote command with postgres storage

// bug: test

// Handle bot start and settings here
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    dotenv().ok();
    let args = Args::parse();

    let mut opt = ConnectOptions::new(args.database_url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(false)
        .sqlx_logging_level(log::LevelFilter::Info);

    let db = Database::connect(opt).await.expect(&format!("Failed to connect to {}", args.database_url));

    let data = Data { db: db.clone() };

    // Run migrations automatically when launched to make sure the DB is setup correctly.
    // todo: Make sure this actually sets up from empty databases down the line so no user setup other than the basics of Postgres are needed.
    Migrator::up(&db, None).await.expect("Failed to run migrations");

    let mut bot_commands = vec![
        age(),
        help(),
        register(),
        ping(),
        info(),
        owo(),
        creationdate(),
        pog(),
        anime(),
        manga()
    ];

    #[cfg(feature = "testing")]
    {
        bot_commands.push(threadtest());
    }

    #[cfg(feature = "postgres")]
    {
        let mut post_features = vec![quotes::getquote(), quotes::addquote(), quotes::randquote()];
        bot_commands.append(&mut post_features);
    }

    let framework = poise::Framework
        ::builder()
        .token(args.token)
        .intents(serenity::GatewayIntents::all() | serenity::GatewayIntents::MESSAGE_CONTENT)
        .setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(data) }))
        .options(poise::FrameworkOptions {
            // configure framework here
            commands: bot_commands,
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("<>".into()),
                ..Default::default()
            },
            ..Default::default()
        });

    framework.run_autosharded().await.unwrap();
}
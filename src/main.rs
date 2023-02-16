use chrono::NaiveDateTime;

use poise::serenity_prelude::{AttachmentType, Colour};

use reqwest::Client;
use rusted_wumpus_lib::checks::user_db_check;
use serde_json::json;

use dotenv::dotenv;
use html2text::from_read;
use sqlx::postgres::PgPoolOptions;
use tracing::instrument;
use tracing::metadata::LevelFilter;
use tracing_unwrap::OptionExt;
// use tracing::{event, Level};
use std::fs::File;
use std::time::Instant;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer};
use tracing_unwrap::ResultExt;

use std::{sync::mpsc, thread};

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
    pub db: sqlx::PgPool,
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
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: HELP_EXTRA_TEXT,
            show_context_menu_commands: true,
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

/// Display your or another user's account creation date
#[poise::command(prefix_command, slash_command, track_edits)]
pub async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let user = user.as_ref().unwrap_or_else(|| ctx.author());
    ctx.say(format!(
        "{}'s account was created at {}",
        user.name,
        user.created_at()
    ))
    .await?;

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
    ctx.say(msg.owoify()).await?;

    Ok(())
}

/// Get an AniList entry for an Anime
#[instrument]
#[poise::command(prefix_command, slash_command, category = "Fun")]
async fn anime(
    ctx: Context<'_>,
    #[description = "Name"] msg: String,
    #[description = "Output raw json"] raw: Option<bool>,
) -> Result<(), Error> {
    // Tell discord wait longer then 3 seconds
    ctx.defer().await?;

    let client = Client::new();

    // Define query and variables
    let json = json!({"query": ANIME_QUERY, "variables": {"search": format!("{msg}")}});

    // Make HTTP post request
    let resp = client
        .post("https://graphql.anilist.co/")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .body(json.to_string())
        .send()
        .await
        .unwrap_or_log()
        .text()
        .await;

    // Get json
    let result: serde_json::Value = serde_json::from_str(&resp.unwrap_or_log()).unwrap_or_log();

    let formatted_json = format!("{result:#?}");

    // let anime_id = result["data"]["Media"]["id"].as_u64().unwrap_or_log();
    let description = from_read(
        result["data"]["Media"]["description"]
            .as_str()
            .unwrap_or_log()
            .as_bytes(),
        50,
    );
    let status = result["data"]["Media"]["status"].as_str().unwrap_or_log();
    let anilist_url = result["data"]["Media"]["siteUrl"].as_str().unwrap_or_log();
    let episode_count = result["data"]["Media"]["episodes"].as_u64().unwrap_or_log();
    let average_episode_length = result["data"]["Media"]["duration"].as_u64().unwrap_or_log();
    let average_score = result["data"]["Media"]["averageScore"].as_u64().unwrap_or_log();
    let median_score = result["data"]["Media"]["meanScore"].as_u64().unwrap_or_log();
    let adult = result["data"]["Media"]["isAdult"].as_bool().unwrap_or_log();

    let romaji_title = result["data"]["Media"]["title"]["romaji"].as_str().unwrap_or_log();
    let english_title = if result["data"]["Media"]["title"]["english"]
        .as_str()
        .is_some()
    {
        result["data"]["Media"]["title"]["english"]
            .as_str()
            .unwrap_or_log()
    } else {
        result["data"]["Media"]["title"]["romaji"].as_str().unwrap_or_log()
    };

    let base_colour = if result["data"]["Media"]["coverImage"]["color"]
        .as_str()
        .is_some()
    {
        result["data"]["Media"]["coverImage"]["color"]
            .as_str()
            .unwrap_or_log()
    } else {
        "#aed6f1"
    };

    let image = result["data"]["Media"]["coverImage"]["extraLarge"]
        .as_str()
        .unwrap_or_log();
    let small_image = result["data"]["Media"]["coverImage"]["large"]
        .as_str()
        .unwrap_or_log();

    let season = if result["data"]["Media"]["season"].as_str().is_some() {
        result["data"]["Media"]["season"].as_str().unwrap_or_log()
    } else {
        "N/A"
    };

    let start_year = if result["data"]["Media"]["startDate"]["year"]
        .as_i64()
        .is_some()
    {
        result["data"]["Media"]["startDate"]["year"]
            .as_i64()
            .unwrap_or_log()
    } else {
        -1
    };
    let start_month = if result["data"]["Media"]["startDate"]["month"]
        .as_i64()
        .is_some()
    {
        result["data"]["Media"]["startDate"]["month"]
            .as_i64()
            .unwrap_or_log()
    } else {
        -1
    };
    let start_day = if result["data"]["Media"]["startDate"]["day"]
        .as_i64()
        .is_some()
    {
        result["data"]["Media"]["startDate"]["day"]
            .as_i64()
            .unwrap_or_log()
    } else {
        -1
    };

    let end_year = if result["data"]["Media"]["endDate"]["year"]
        .as_i64()
        .is_some()
    {
        result["data"]["Media"]["endDate"]["year"].as_i64().unwrap_or_log()
    } else {
        -1
    };
    let end_month = if result["data"]["Media"]["endDate"]["month"]
        .as_i64()
        .is_some()
    {
        result["data"]["Media"]["endDate"]["month"]
            .as_i64()
            .unwrap_or_log()
    } else {
        -1
    };
    let end_day = if result["data"]["Media"]["endDate"]["day"].as_i64().is_some() {
        result["data"]["Media"]["endDate"]["day"].as_i64().unwrap_or_log()
    } else {
        -1
    };

    let without_prefix = base_colour.trim_start_matches('#');
    let colour_i32 = i32::from_str_radix(without_prefix, 16).unwrap_or_log();

    let field_list = [
        ("English Name", english_title.to_string(), true),
        ("Romaji Name", romaji_title.to_string(), true),
        ("Description", description.to_string(), false),
        (
            "Start Date",
            format!("{season} {start_year}/{start_month}/{start_day}"),
            true,
        ),
        (
            "End Date",
            format!("{end_year}/{end_month}/{end_day}"),
            true,
        ),
        ("Status", status.to_string(), true),
        ("Episode Count", format!("{episode_count}"), true),
        (
            "Episode Length",
            format!("{average_episode_length} minutes"),
            true,
        ),
        ("Average score", format!("{average_score}"), true),
        ("Mean score", format!("{median_score}"), true),
        ("Is adult?", format!("{adult}"), true),
    ];

    if raw.is_some() {
        if raw.unwrap_or_log() {
            ctx.send(|f| {
                f.content("Anime result")
                    .ephemeral(false)
                    .attachment(AttachmentType::Bytes {
                        data: std::borrow::Cow::Borrowed(formatted_json.as_bytes()),
                        filename: String::from("Anime.json"),
                    })
            })
            .await?;
        } else {
            ctx.send(|f| {
                f.embed(|b| {
                    b.colour(Colour::from(colour_i32).tuple())
                        .description("Anime Result")
                        .image(image)
                        .author(|f| f.icon_url(small_image).name("AniList").url(anilist_url))
                        .fields(field_list)
                })
            })
            .await?;
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
        })
        .await?;
    }
    Ok(())
}

/// Get an AniList entry for a Manga
#[instrument]
#[poise::command(prefix_command, slash_command, category = "Fun")]
async fn manga(
    ctx: Context<'_>,
    #[description = "Name"] msg: String,
    #[description = "Output raw json"] raw: Option<bool>,
) -> Result<(), Error> {
    // Tell discord wait longer then 3 seconds
    ctx.defer().await?;

    let client = Client::new();

    // Define query and variables
    let json = json!({"query": MANGA_QUERY, "variables": {"search": format!("{msg}")}});

    // Make HTTP post request
    let resp = client
        .post("https://graphql.anilist.co/")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .body(json.to_string())
        .send()
        .await
        .unwrap_or_log()
        .text()
        .await;

    // Get json
    let result: serde_json::Value = serde_json::from_str(&resp.unwrap_or_log()).unwrap_or_log();

    let formatted_json = format!("{result:#?}");

    if raw.is_some() && raw.unwrap_or_log() {
        ctx.send(|f| {
            f.content("Anime result")
                .ephemeral(false)
                .attachment(AttachmentType::Bytes {
                    data: std::borrow::Cow::Borrowed(formatted_json.as_bytes()),
                    filename: String::from("Anime.json"),
                })
        })
        .await?;

        return Ok(());
    }

    // let anime_id = result["data"]["Media"]["id"].as_u64().unwrap_or_log();
    let description = from_read(
        result["data"]["Media"]["description"]
            .as_str()
            .unwrap_or_log()
            .as_bytes(),
        50,
    );
    let status = result["data"]["Media"]["status"].as_str().unwrap_or_log();
    let anilist_url = result["data"]["Media"]["siteUrl"].as_str().unwrap_or_log();
    let volume_count = if result["data"]["Media"]["volumes"].as_i64().is_some() {
        result["data"]["Media"]["volumes"].as_i64().unwrap_or_log()
    } else {
        -1
    };
    let chapter_coumt = result["data"]["Media"]["chapters"].as_u64().unwrap_or_log();
    let average_score = result["data"]["Media"]["averageScore"].as_u64().unwrap_or_log();
    let median_score = result["data"]["Media"]["meanScore"].as_u64().unwrap_or_log();
    let adult = result["data"]["Media"]["isAdult"].as_bool().unwrap_or_log();

    let romaji_title = result["data"]["Media"]["title"]["romaji"].as_str().unwrap_or_log();
    let english_title = if result["data"]["Media"]["title"]["english"]
        .as_str()
        .is_some()
    {
        result["data"]["Media"]["title"]["english"]
            .as_str()
            .unwrap_or_log()
    } else {
        result["data"]["Media"]["title"]["romaji"].as_str().unwrap_or_log()
    };

    let base_colour = if result["data"]["Media"]["coverImage"]["color"]
        .as_str()
        .is_some()
    {
        result["data"]["Media"]["coverImage"]["color"]
            .as_str()
            .unwrap_or_log()
    } else {
        "#aed6f1"
    };

    let image = result["data"]["Media"]["coverImage"]["extraLarge"]
        .as_str()
        .unwrap_or_log();
    let small_image = result["data"]["Media"]["coverImage"]["large"]
        .as_str()
        .unwrap_or_log();

    let season = if result["data"]["Media"]["season"].as_str().is_some() {
        result["data"]["Media"]["season"].as_str().unwrap_or_log()
    } else {
        "N/A"
    };

    let start_year = if result["data"]["Media"]["startDate"]["year"]
        .as_i64()
        .is_some()
    {
        result["data"]["Media"]["startDate"]["year"]
            .as_i64()
            .unwrap_or_log()
    } else {
        -1
    };
    let start_month = if result["data"]["Media"]["startDate"]["month"]
        .as_i64()
        .is_some()
    {
        result["data"]["Media"]["startDate"]["month"]
            .as_i64()
            .unwrap_or_log()
    } else {
        -1
    };
    let start_day = if result["data"]["Media"]["startDate"]["day"]
        .as_i64()
        .is_some()
    {
        result["data"]["Media"]["startDate"]["day"]
            .as_i64()
            .unwrap_or_log()
    } else {
        -1
    };

    let end_year = if result["data"]["Media"]["endDate"]["year"]
        .as_i64()
        .is_some()
    {
        result["data"]["Media"]["endDate"]["year"].as_i64().unwrap_or_log()
    } else {
        -1
    };
    let end_month = if result["data"]["Media"]["endDate"]["month"]
        .as_i64()
        .is_some()
    {
        result["data"]["Media"]["endDate"]["month"]
            .as_i64()
            .unwrap_or_log()
    } else {
        -1
    };
    let end_day = if result["data"]["Media"]["endDate"]["day"].as_i64().is_some() {
        result["data"]["Media"]["endDate"]["day"].as_i64().unwrap_or_log()
    } else {
        -1
    };

    let without_prefix = base_colour.trim_start_matches('#');
    let colour_i32 = i32::from_str_radix(without_prefix, 16).unwrap_or_log();

    let field_list = [
        ("English Name", english_title.to_string(), true),
        ("Romaji Name", romaji_title.to_string(), true),
        ("Description", description.to_string(), false),
        (
            "Start Date",
            format!("{season} {start_year}/{start_month}/{start_day}"),
            true,
        ),
        (
            "End Date",
            format!("{end_year}/{end_month}/{end_day}"),
            true,
        ),
        ("Status", status.to_string(), true),
        ("Volume Count", format!("{volume_count}"), true),
        ("Chapter Count", format!("{chapter_coumt} minutes"), true),
        ("Average Score", format!("{average_score}"), true),
        ("Mean Score", format!("{median_score}"), true),
        ("Is Adult?", format!("{adult}"), true),
    ];

    ctx.send(|f| {
        f.embed(|b| {
            b.colour(Colour::from(colour_i32).tuple())
                .description("Anime Result")
                .image(image)
                .author(|f| f.icon_url(small_image).name("AniList").url(anilist_url))
                .fields(field_list)
        })
    })
    .await?;
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
        tx1.send(channel_msg).unwrap_or_log(); // Send math over channel 1
        println!("Sent {channel_msg} on channel 1!"); // Print once channel 1 takes the message

        let duration = start.elapsed().as_nanos() as f64 / 1000000_f64; // End time tracking
        tx3.send(duration).unwrap_or_log(); // Send the ms taken
    });

    thread::spawn(move || {
        let start = Instant::now();
        let channel_msg = 420 * 2;
        tx2.send(channel_msg).unwrap_or_log();
        println!("Sent {channel_msg} on channel 2!");
        let duration = start.elapsed().as_nanos() as f64 / 1000000_f64;
        tx4.send(duration).unwrap_or_log();
    });

    ctx.say(format!(
        "Thread 1 returned: {}\nThread 2 returned: {}",
        rx1.recv().unwrap_or_log(),
        rx2.recv().unwrap_or_log()
    ))
    .await?; // This line wont actually complete until both threads are firing in their channels

    if timed
    // <>threadtest -t
    {
        ctx.say(format!(
            "Thread 1 took {}ms to complete\nThread 2 took {}ms to complete",
            rx3.recv().unwrap_or_log(),
            rx4.recv().unwrap_or_log()
        ))
        .await?;
    } else {
        // I'm just throwing away these channels unless being called since this is a test command. probably wouldn't leave the time tracking in at all if this was a more functional command
        let _ = rx3.recv().unwrap_or_log();
        let _ = rx4.recv().unwrap_or_log();
    }

    Ok(())
}

/// Gets the creation date or a Snowflake ID
#[poise::command(prefix_command, slash_command, category = "Tools")]
async fn creationdate(
    ctx: Context<'_>,
    #[description = "ID"] snowflake_id: u128,
) -> Result<(), Error> {
    let unix_timecode = snowflake_to_unix(snowflake_id);

    let date_time_stamp = NaiveDateTime::from_timestamp_opt(unix_timecode as i64, 0);

    if date_time_stamp.is_none() {
        ctx.say("Unable to retrieve timestamp from snowflake")
            .await?;
    } else {
        ctx.say(format!("Created/Joined on {}", date_time_stamp.unwrap_or_log()))
            .await?;
    }

    Ok(())
}

// Place other functions bellow here

/// Converts a dsicord snowflake to a unix timecode
const fn snowflake_to_unix(id: u128) -> u128 {
    const DISCORD_EPOCH: u128 = 1420070400000;

    ((id >> 22) + DISCORD_EPOCH) / 1000
}

// Handle bot start and settings here
#[tokio::main]
async fn main() {
    let console_layer = tracing_subscriber::fmt::layer()
        .with_line_number(true)
        .with_ansi(true)
        .with_thread_names(true)
        .with_target(true)
        .with_filter(LevelFilter::INFO);

    let info_file_layer = match File::create(
        std::path::Path::new(&std::env::current_dir().unwrap_or_log()).join(format!(
            "./{}-rusted_wumpus.info.log",
            chrono::offset::Local::now().timestamp()
        )),
    ) {
        Ok(handle) => {
            let file_log = tracing_subscriber::fmt::layer()
                .with_line_number(true)
                .with_ansi(false)
                .with_thread_names(true)
                .with_target(true)
                .with_writer(handle)
                .with_filter(LevelFilter::INFO);
            Some(file_log)
        }
        Err(why) => {
            eprintln!("ERROR!: Unable to create log output file: {why:?}");
            None
        }
    };

    tracing_subscriber::registry()
        .with(console_layer)
        .with(info_file_layer)
        .init();

    dotenv().ok();
    let args = Args::parse();

    // Create a DB connection and embed it into the data struct for poise
    let db = PgPoolOptions::new()
        .max_connections(100)
        .connect(&args.database_url)
        .await
        .expect_or_log("Unable to connect to the DB!");
    let data = Data { db: db.clone() };

    // Run migrations automatically when launched to make sure the DB is setup correctly.
    // todo: Make sure this actually sets up from empty databases down the line so no user setup other than the basics of Postgres are needed.
    sqlx::migrate!("./migrations/")
        .run(&db)
        .await
        .expect_or_log("Failed to run migrations");

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
        manga(),
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

    let framework = poise::Framework::builder()
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
            pre_command: |ctx| {
                Box::pin(async move {
                    // This will add the user to the `users` table if they aren't there already
                    user_db_check(ctx.data().db.clone(), ctx.author().clone()).await;
                })
            },
            ..Default::default()
        });

    framework.run().await.unwrap_or_log();
}

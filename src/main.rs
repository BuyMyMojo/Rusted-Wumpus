use chrono::NaiveDateTime;

use std::time::Instant;

use std::{sync::mpsc, thread}; // Multithreading // Time tracking

use owoify::OwOifiable;

use poise::serenity_prelude::{self as serenity};

type Data = ();
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

// I wounder if storing this text as a const is more efficient then just putting it inside the reply function? I will ask around later.
const INFO_MESSAGE: &str = "
Hello there, Human!

This is just an example message I am making as a test for this bot!

— RustBot 🤖
";

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
            extra_text_at_bottom: "\
This is a test bot I made to learn Rust",
            show_context_menu_commands: true,
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    poise::Framework::build()
        .token(std::env::var("TESTING_DISCORD_TOKEN").expect("Expected a token in the environment"))
        .user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(()) }))
        .options(poise::FrameworkOptions {
            // configure framework here
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("<>".into()),
                ..Default::default()
            },
            commands: vec![
                age(),
                help(),
                register(),
                ping(),
                info(),
                owo(),
                threadtest(),
                creationdate(),
            ],
            ..Default::default()
        })
        .run()
        .await
        .unwrap()
}

// Create commands bellow!

/// Display your or another user's account creation date
#[poise::command(prefix_command, slash_command, track_edits)]
pub async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let user = user.as_ref().unwrap_or(ctx.author());
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
#[poise::command(prefix_command, hide_in_help, owners_only)]
async fn register(ctx: Context<'_>, #[flag] global: bool) -> Result<(), Error> {
    poise::builtins::register_application_commands(ctx, global).await?;

    Ok(())
}

/// Replies with pong!
#[poise::command(prefix_command, slash_command, category = "Miscellaneous")]
async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Pong!").await?;

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
async fn owo(
    ctx: Context<'_>,
    #[description = "Message"] msg: Option<String>,
) -> Result<(), Error> {
    ctx.say(String::from(msg.unwrap()).owoify()).await?;

    Ok(())
}

/// Tests multithreded functionality. use -t to show how long the threads live for
#[poise::command(prefix_command, slash_command, category = "Testing")]
async fn threadtest(ctx: Context<'_>, #[description = "Timed"] timed: bool) -> Result<(), Error> {
    // Main math channels
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    // Optional duration cahnnels
    let (tx3, rx3) = mpsc::channel();
    let (tx4, rx4) = mpsc::channel();

    thread::spawn(move || {
        let start = Instant::now(); // Start time tracking

        let channel_msg = 69 + 420; // Math
        tx1.send(channel_msg).unwrap(); // Send math over channel 1
        println!("Sent {} on channel 1!", channel_msg); // Print once channel 1 takes the message

        let duration = start.elapsed().as_nanos() as f64 / 1000000 as f64; // End time tracking
        tx3.send(duration).unwrap(); // Send the ms taken
    });

    thread::spawn(move || {
        let start = Instant::now();
        let channel_msg = 420 * 2;
        tx2.send(channel_msg).unwrap();
        println!("Sent {} on channel 2!", channel_msg);
        let duration = start.elapsed().as_nanos() as f64 / 1000000 as f64;
        tx4.send(duration).unwrap();
    });

    ctx.say(format!(
        "Thread 1 returned: {}\nThread 2 returned: {}",
        rx1.recv().unwrap(),
        rx2.recv().unwrap()
    ))
    .await?; // This line wont actually complete until both threads are firing in their channels

    if timed
    // <>threadtest -t
    {
        ctx.say(format!(
            "Thread 1 took {}ms to complete\nThread 2 took {}ms to complete",
            rx3.recv().unwrap(),
            rx4.recv().unwrap()
        ))
        .await?;
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
    #[description = "ID"] snowflake_id: u128,
) -> Result<(), Error> {
    let unix_timecode = snowflake_to_unix(snowflake_id);

    let date_time_stamp = NaiveDateTime::from_timestamp(unix_timecode as i64, 0);

    ctx.say(format!("Created/Joined on {}", date_time_stamp))
        .await?;

    Ok(())
}

// Place other functions bellow here

/// Converts a dsicord snowflake to a unix timecode
fn snowflake_to_unix(id: u128) -> u128 {
    const DISCORD_EPOCH: u128 = 1420070400000;

    let unix_timecode = ((id >> 22) + DISCORD_EPOCH) / 1000;

    return unix_timecode;
}
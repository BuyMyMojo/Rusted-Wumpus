use std::{collections::HashSet, env};

use serenity::{
    async_trait,
    framework::standard::{
        help_commands,
        macros::{command, group, help},
        Args,
        CommandGroup,
        CommandResult,
        HelpOptions,
        StandardFramework,
    },
    model::prelude::*,
    prelude::*,
};

// I wounder if storing this text as a const is more efficient then just putting it inside the reply function? I will ask around later.
const INFO_MESSAGE: &str = "
Hello there, Human!

This is just an example message I am making as a test for this bot!

— RustBot 🤖
";

#[group]
#[commands(ping, info)]  // Do I actually need to list all my commands here??
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // async fn message(&self, ctx: Context, msg: Message) {
        
    // }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("Bot running as {}", ready.user.name);
    }
}

// Define the help command
#[help]
async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("<>")) // set the bot's prefix to "<>"
        .help(&MY_HELP)
        .group(&GENERAL_GROUP);

    // Grab my testing token from the env variables
    let token = env::var("TESTING_DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    // Create the client using the Handler created earlier
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Err creating client");

    // Start the bot
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}


// Create commands bellow!

#[command("ping")]
#[description("Replies with 'Pong!'")]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

#[command("info")]
#[description("Replies with some basic info")]
async fn info(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, INFO_MESSAGE).await?;

    Ok(())
}

// Probabbly place other functions bellow here

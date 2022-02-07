use std::{collections::HashSet, env};
use std::{thread, sync::mpsc};  // Multithreading
use std::time::Instant;  // Time tracking

use owoify::OwOifiable;

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
#[commands(ping, info, owo, threadtest)]  // Do I actually need to list all my commands here??
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
async fn ping(ctx: &Context, msg: &Message) -> CommandResult 
{
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

#[command("info")]
#[description("Replies with some basic info")]
async fn info(ctx: &Context, msg: &Message) -> CommandResult 
{
    msg.reply(ctx, INFO_MESSAGE).await?;

    Ok(())
}

#[command("owo")]
#[description("OwOifys your message")]
async fn owo(ctx: &Context, msg: &Message) -> CommandResult 
{
    let text = String::from(
        msg.content
        .trim_start_matches("<>owo ")  // Remove the start of the command. proabbly a way to get the message without removing the start, like Nextcord's * args. too tired to look into it
    );

    match msg.content.as_str() {
        "<>owo" => msg.reply(ctx, "You must provide input text!").await?,
        _ => msg.reply(ctx,text.owoify()).await?,
    };

    Ok(())
}


#[command("threadtest")]
#[description("Tests multithreded functionality. use -t to show how long the threads live for")]
#[example("[-t]")]
async fn threadtest(ctx: &Context, msg: &Message) -> CommandResult {
    // Main math channels
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    // Optional duration cahnnels
    let (tx3, rx3) = mpsc::channel();
    let (tx4, rx4) = mpsc::channel();

    thread::spawn(move || {
        let start = Instant::now();  // Start time tracking

        let channel_msg = 69 + 420;  // Math
        tx1.send(channel_msg).unwrap();  // Send math over channel 1
        println!("Sent {} on channel 1!", channel_msg);  // Print once channel 1 takes the message
        
        let duration = start.elapsed().as_nanos() as f64 / 1000000 as f64;  // End time tracking
        tx3.send(duration).unwrap();  // Send the ms taken
    });

    thread::spawn(move || {
        let start = Instant::now();
        let channel_msg = 420 * 2;
        tx2.send(channel_msg).unwrap();
        println!("Sent {} on channel 2!", channel_msg);
        let duration = start.elapsed().as_nanos() as f64 / 1000000 as f64;
        tx4.send(duration).unwrap();
    });

    msg.reply(ctx, format!("Thread 1 returned: {}\nThread 2 returned: {}", rx1.recv().unwrap(), rx2.recv().unwrap())).await?;

    if msg.content.trim_start_matches("<>threadtest ") == "-t"
    {
    msg.reply(ctx, format!("Thread 1 took {}ms to complete\nThread 2 took {}ms to complete", rx3.recv().unwrap(), rx4.recv().unwrap())).await?;
    } else {  // I'm just throwing away these channels unless being called since this is a test command. probably wouldn't leave the time tracking in at all if this was a more functional command
        let _ = rx3.recv().unwrap();
        let _ = rx4.recv().unwrap();
    }

    Ok(())
}

// Probabbly place other functions bellow here

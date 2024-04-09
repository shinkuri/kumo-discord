mod commands;

use serenity::all::CreateInteractionResponse;
use serenity::all::CreateInteractionResponseMessage;
use serenity::all::GuildId;
use serenity::all::Interaction;
use serenity::all::Ready;
use serenity::async_trait;
// for global commands
//use serenity::model::application::command::Command;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            println!("Received command interaction: {command:#?}");
            let content = match command.data.name.as_str() {
                "ping" => commands::ping::run(&command.data.options),
                "timer" => commands::timer::run(&command.data.options),
                "stampy" => commands::stampy::run(&command.data.options),
                // More commands go here
                _ => "not implemented :(".to_string(),
            };
            if let Err(why) = command.create_response(&ctx.http, 
                CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(content)))
                .await
            {
                println!("Cannot respond to slash command: {why}");
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        let guild_id = GuildId::new(
            std::env::var("GUILD_ID")
                .expect("GUILD_ID must be set.")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );
        let commands = guild_id.set_commands(&ctx.http, vec![
            commands::ping::register(),
            commands::timer::register(),
            commands::stampy::register(),
            // More commands go here
        ])
        .await;
        println!("I now have the following guild slash commands: {commands:#?}`");
        // let guild_command = Command::create_global_application_command(&ctx.http, |command| {
        //    commands::ping::register(command)
        //})
        //.await;
        //println!("I created the following global slash command: {:#?}", guild_command);
    }
}

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN missing");

    let intents = GatewayIntents::GUILD_MESSAGES
    | GatewayIntents::DIRECT_MESSAGES
    | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Failed to create client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}

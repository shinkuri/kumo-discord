use serenity::builder::CreateCommand;
use serenity::model::application::CommandDataOption;


use std::time::{Duration, Instant};

pub fn run(_options: &[CommandDataOption]) -> String {
    // TODO grab paramets

    let when = Instant::now() + Duration::from_secs(1);
    

    "Timer feature under construction!".to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("timer").description("Timer command")
}
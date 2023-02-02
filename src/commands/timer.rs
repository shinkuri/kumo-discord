use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;


use std::time::{Duration, Instant};

pub fn run(_options: &[CommandDataOption]) -> String {
    // TODO grab paramets

    let when = Instant::now() + Duration::from_secs(1);
    

    "Timer feature under construction!".to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("timer").description("Timer command")
}
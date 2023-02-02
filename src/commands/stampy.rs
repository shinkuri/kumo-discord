use chrono::{Utc, Duration};

use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn run(_options: &[CommandDataOption]) -> String {
    let utc = Utc::now().naive_utc();
    
    let then = utc.checked_add_signed(Duration::minutes(8 * 60 + 4 * 6));

    return match then {
        Some(result) => format!("<t:{}>", result.timestamp()),
        None => "Process failed".to_string()
    };
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("stampy").description("Add 8.4h to current time")
}
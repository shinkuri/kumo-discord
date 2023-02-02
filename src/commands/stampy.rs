use chrono::{TimeZone, Utc, Duration};
use chrono_tz::Europe::Berlin;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn run(_options: &[CommandDataOption]) -> String {
    let utc = Utc::now().naive_utc();
    let berlin_time = Berlin.from_utc_datetime(&utc)
            .checked_add_signed(Duration::minutes(8 * 60 + 4 * 6)); // adds 8.4h

    return match berlin_time {
        Some(result) => result.to_string(),
        None => "Process failed".to_string()
    };
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("stampy").description("Add 8.4h to current time")
}
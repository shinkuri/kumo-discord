use chrono::{TimeZone, Utc, Duration};
use chrono_tz::Europe::Berlin;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn run(_options: &[CommandDataOption]) -> String {
    // TODO grab paramets

    let utc = Utc::now().naive_utc();
    let berlin_time = Berlin.from_utc_datetime(&utc);
    
    berlin_time.checked_add_signed(Duration::minutes(8 * 60 + 4 * 6)); // add 8.4h

    berlin_time.to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("stampy").description("Add 7.9h to current time")
}
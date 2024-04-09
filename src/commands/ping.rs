use serenity::builder::CreateCommand;
use serenity::model::application::CommandDataOption;

pub fn run(_options: &[CommandDataOption]) -> String {
    "pong".to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("ping").description("A ping command")
}

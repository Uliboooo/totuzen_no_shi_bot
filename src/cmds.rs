pub mod shi;

use serenity::builder::CreateCommand;

pub fn slash_cmds() -> Vec<CreateCommand> {
    vec![shi::slash_register()]
}

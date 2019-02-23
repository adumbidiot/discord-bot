use serenity::{
    client::Context,
    framework::standard::{
        Args,
        Command,
        CommandError,
        CommandOptions,
    },
    model::channel::Message,
};
use std::sync::Arc;

pub struct Ping {
    opts: Arc<CommandOptions>,
}

impl Ping {
    pub fn new() -> Self {
        let mut opts = CommandOptions::default();
        opts.desc = Some(String::from("Respond with pong"));

        Ping {
            opts: Arc::from(opts),
        }
    }
}

impl Command for Ping {
    fn execute(&self, _: &mut Context, msg: &Message, _: Args) -> Result<(), CommandError> {
        msg.channel_id.say("pong")?;
        Ok(())
    }

    fn options(&self) -> Arc<CommandOptions> {
        self.opts.clone()
    }
}

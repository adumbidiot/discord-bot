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
use xkcd::Client;

pub struct Xkcd {
    opts: Arc<CommandOptions>,
    client: Client,
}

impl Xkcd {
    pub fn new() -> Self {
        let mut opts = CommandOptions::default();
        opts.desc = Some(String::from("Get a random comic from Xkcd"));

        Xkcd {
            opts: Arc::from(opts),
            client: Client::new(),
        }
    }
}

impl Command for Xkcd {
    fn execute(&self, _: &mut Context, msg: &Message, _: Args) -> Result<(), CommandError> {
        let res = match self.client.get_random() {
            Ok(data) => data,
            Err(e) => format!("{:#?}", e),
        };

        msg.channel_id.say(res)?;
        Ok(())
    }

    fn options(&self) -> Arc<CommandOptions> {
        self.opts.clone()
    }
}

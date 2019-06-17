use nekos::Client;
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

pub struct Nekos {
    opts: Arc<CommandOptions>,
    client: Client,
}

impl Nekos {
    pub fn new() -> Self {
        let mut opts = CommandOptions::default();
        opts.desc = Some(String::from("Get a random catgirl image"));

        Nekos {
            opts: Arc::from(opts),
            client: Client::new(),
        }
    }
}

impl Command for Nekos {
    fn execute(&self, _: &mut Context, msg: &Message, mut args : Args) -> Result<(), CommandError> {
		let mut nsfw = false;
		let count = 1;
		
		if args.single::<String>().map(|s| s == "--nsfw").unwrap_or(false) {
			nsfw = true;
		}
		
        let res = match self.client.get_random_images(nsfw, count) {
            Ok(data) => format!("https://nekos.moe/image/{}", data.images[0].id),
            Err(e) => format!("{:#?}", e),
        };

        msg.channel_id.say(res)?;
        Ok(())
    }

    fn options(&self) -> Arc<CommandOptions> {
        self.opts.clone()
    }
}

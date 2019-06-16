use fml::{
    Article,
    Client,
};
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

// TODO: Caching
pub struct Fml {
    opts: Arc<CommandOptions>,
    client: Arc<Client>,
}

impl Fml {
    pub fn new(client: Arc<Client>) -> Self {
        let mut opts = CommandOptions::default();
        opts.desc = Some(String::from("Get a random story from fmylife.com"));

        Fml {
            opts: Arc::from(opts),
            client,
        }
    }
}

impl Command for Fml {
    fn execute(&self, _: &mut Context, msg: &Message, _: Args) -> Result<(), CommandError> {
        let res = match self.client.list_random(1) {
            Ok(data) => format_entry(&data[0]),
            Err(e) => format!("{:#?}", e),
        };

        msg.channel_id.say(res)?;
        Ok(())
    }

    fn options(&self) -> Arc<CommandOptions> {
        self.opts.clone()
    }
}

pub fn format_entry(entry: &Article) -> String {
    let mut ret = entry.content_hidden.clone() + "\n";
    ret.push_str(&format!(
        "I agree, your life sucks: {}\n",
        entry.metrics.votes_up
    ));
    ret.push_str(&format!("You deserved it: {}\n", entry.metrics.votes_down));
    ret.push_str(&format!("😐: {}\n", entry.metrics.smiley_amusing));
    ret.push_str(&format!("😃: {}\n", entry.metrics.smiley_funny));
    ret.push_str(&format!("😲: {}\n", entry.metrics.smiley_weird));
    ret.push_str(&format!("😂: {}\n", entry.metrics.smiley_hilarious));
    ret
}

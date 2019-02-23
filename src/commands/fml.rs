use fml::{
    Client,
    FmlEntry,
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

pub struct Fml {
    opts: Arc<CommandOptions>,
    client: Client,
}

impl Fml {
    pub fn new() -> Self {
        let mut opts = CommandOptions::default();
        opts.desc = Some(String::from("Get a random story from fmylife.com"));

        Fml {
            opts: Arc::from(opts),
            client: Client::new(),
        }
    }
}

impl Command for Fml {
    fn execute(&self, _: &mut Context, msg: &Message, _: Args) -> Result<(), CommandError> {
        let res = match self.client.get_random() {
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

pub fn format_entry(entry: &FmlEntry) -> String {
    let mut ret = entry.text.clone() + "\n";
    ret.push_str(&format!("I agree, your life sucks: {}\n", entry.sucks));
    ret.push_str(&format!("You deserved it: {}\n", entry.deserved));
    ret.push_str(&format!("😐: {}\n", entry.amusing));
    ret.push_str(&format!("😃: {}\n", entry.funny));
    ret.push_str(&format!("😲: {}\n", entry.weird));
    ret.push_str(&format!("😂: {}\n", entry.hilarious));
    ret
}

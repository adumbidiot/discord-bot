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
use urban::Client;

pub struct Urban {
    opts: Arc<CommandOptions>,
    client: Client,
}

impl Urban {
    pub fn new() -> Self {
        let mut opts = CommandOptions::default();
        opts.desc = Some(String::from(
            "Get the top defenition from UrbanDictionary.com",
        ));
        opts.min_args = Some(1);
        opts.max_args = Some(1);

        Urban {
            opts: Arc::from(opts),
            client: Client::new(),
        }
    }
}

impl Command for Urban {
    fn execute(&self, _: &mut Context, msg: &Message, mut args: Args) -> Result<(), CommandError> {
        let word: String = args.single_quoted()?;
        let res = match self.client.lookup(&word) {
            Ok(data) => format_defenition(&data.list[0]),
            Err(e) => format!("{:#?}", e),
        };

        msg.channel_id.say(res)?;
        Ok(())
    }

    fn options(&self) -> Arc<CommandOptions> {
        self.opts.clone()
    }
}

fn format_defenition(def: &urban::Definition) -> String {
    let mut ret = String::new();
	ret += &format!("Word: {}\n", def.word);
    ret += &format!("Definition: {}\n", def.definition);
	ret += &format!("Example: {}\n", def.example);
	ret += &format!("👍: {}\n", def.thumbs_up);
	ret += &format!("👎: {}\n", def.thumbs_down);
	ret += &format!("Link: {}", def.permalink);
    ret
}

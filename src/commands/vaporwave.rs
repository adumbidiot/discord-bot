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

pub struct Vaporwave {
    opts: Arc<CommandOptions>,
}

impl Vaporwave {
    pub fn new() -> Self {
        let mut opts = CommandOptions::default();
        opts.desc = Some(String::from("Vaporwave a phrase"));
        opts.usage = Some(String::from(r#""<phrase>""#));
        opts.example = Some(String::from(r#""Hello World!""#));
        opts.min_args = Some(1);

        Vaporwave {
            opts: Arc::from(opts),
        }
    }
}

impl Command for Vaporwave {
    fn execute(&self, _: &mut Context, msg: &Message, mut args: Args) -> Result<(), CommandError> {
        msg.channel_id
            .say(vaporwave(&args.single_quoted::<String>().unwrap()))?;
        Ok(())
    }

    fn options(&self) -> Arc<CommandOptions> {
        self.opts.clone()
    }
}

fn vaporwave(data: &str) -> String {
    data.chars()
        .filter_map(|c| {
            let c = c as u32;
            if c >= 33 && c <= 270 {
                std::char::from_u32(c + 65248)
            } else {
                Some(32 as char)
            }
        })
        .collect()
}

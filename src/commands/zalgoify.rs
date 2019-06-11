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
use zalgo::{
    RandOrStatic,
    Zalgoifier,
};

pub struct Zalgoify {
    opts: Arc<CommandOptions>,
}

impl Zalgoify {
    pub fn new() -> Self {
        let mut opts = CommandOptions::default();
        opts.desc = Some(String::from("Zalgoify a phrase"));
        opts.usage = Some(String::from(r#""<phrase>"<Max Length>"#));
        opts.example = Some(String::from(r#""Hello World!" 50"#));
        opts.min_args = Some(1);
        Zalgoify {
            opts: Arc::from(opts),
        }
    }
}

impl Command for Zalgoify {
    fn execute(&self, _: &mut Context, msg: &Message, mut args: Args) -> Result<(), CommandError> {
        let input: String = args.single_quoted()?;
        let input_max = args.single().unwrap_or(2000);

        let input_len = input.chars().count();
        let total = (input_max as f32 - input_len as f32) / input_len as f32;
        let max = (total / 3.0) as usize;

        if max == 0 {
            msg.channel_id
                .say("The phrase cannot be zalgoified within the given limits")?;
            return Ok(());
        }

        let mut zalgoifier = Zalgoifier::new();
        zalgoifier.set_up(RandOrStatic::Static(max));
        zalgoifier.set_down(RandOrStatic::Static(max));
        zalgoifier.set_mid(RandOrStatic::Static(max));
        let output = zalgoifier.zalgoify(&input);

        msg.channel_id.say(&output)?;

        Ok(())
    }

    fn options(&self) -> Arc<CommandOptions> {
        self.opts.clone()
    }
}

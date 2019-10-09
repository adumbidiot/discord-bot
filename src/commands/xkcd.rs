use crate::XkcdKey;
use serenity::{
    client::Context,
    framework::standard::{
        macros::command,
        Args,
        CommandResult,
    },
    model::channel::Message,
};

#[command]
#[description("Get a random comic from Xkcd")]
fn xkcd(ctx: &mut Context, msg: &Message, _args: Args) -> CommandResult {
    let xkcd_client = ctx.data.read().get::<XkcdKey>().unwrap().clone();

    let res = match xkcd_client.get_random() {
        Ok(data) => data,
        Err(e) => format!("{:#?}", e),
    };

    msg.channel_id.say(&ctx.http, res)?;
    Ok(())
}

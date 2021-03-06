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
#[description("Respond with pong")]
fn ping(ctx: &mut Context, msg: &Message, _args: Args) -> CommandResult {
    msg.channel_id.say(&ctx.http, "pong")?;
    Ok(())
}

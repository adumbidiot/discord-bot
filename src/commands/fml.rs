use crate::FmlKey;
use fml::Article;
use serenity::{
    client::Context,
    framework::standard::{
        macros::command,
        Args,
        CommandResult,
    },
    model::channel::Message,
};

// TODO: Caching
// TODO: Finish formatting command output
#[command]
#[description("Get a random story from fmylife.com")]
fn fml(ctx: &mut Context, msg: &Message, _args: Args) -> CommandResult {
    let data_lock = ctx.data.read();
    let fml_client = data_lock.get::<FmlKey>().unwrap();
    let res = match fml_client.list_random(1) {
        Ok(data) => format_entry(&data[0]),
        Err(e) => format!("{:#?}", e),
    };
    msg.channel_id.say(&ctx.http, res)?;
    Ok(())
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

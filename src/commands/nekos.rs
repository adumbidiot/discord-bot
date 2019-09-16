use crate::NekosKey;
use nekos::{
    Client,
    Image,
    NekosError,
};
use serenity::{
    client::Context,
    framework::standard::{
        macros::command,
        Args,
        CommandResult,
    },
    model::channel::Message,
    prelude::RwLock,
};
use std::sync::Arc;

type ShareVec = Arc<RwLock<Vec<Image>>>;

pub struct NekosClient {
    client: Client,
    nsfw_buffer: ShareVec,
    buffer: ShareVec,
}

impl NekosClient {
    pub fn new() -> Self {
        NekosClient {
            client: Client::new(),
            buffer: Arc::from(RwLock::from(Vec::with_capacity(100))),
            nsfw_buffer: Arc::from(RwLock::from(Vec::with_capacity(100))),
        }
    }

    fn get_buffer(&self, nsfw: bool) -> ShareVec {
        if nsfw {
            self.nsfw_buffer.clone()
        } else {
            self.buffer.clone()
        }
    }

    fn get_image_from_buffer(&self, nsfw: bool) -> Option<Image> {
        self.get_buffer(nsfw).write().pop()
    }

    fn populate_buffer(&self, nsfw: bool) -> Result<(), NekosError> {
        self.client.get_random_images(nsfw, 100).map(|image_list| {
            self.get_buffer(nsfw)
                .write()
                .extend_from_slice(&image_list.images)
        })
    }
}

#[command]
#[description("Get a random catgirl image")]
fn nekos(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let nsfw = args
        .single::<String>()
        .map(|s| s == "--nsfw")
        .unwrap_or(false);

    let data_lock = ctx.data.read();
    let nekos_client = data_lock.get::<NekosKey>().unwrap();

    if let Some(image) = nekos_client.get_image_from_buffer(nsfw) {
        msg.channel_id
            .say(&ctx.http, format!("https://nekos.moe/image/{}", image.id))?;
    } else {
        match nekos_client.populate_buffer(nsfw) {
            Ok(()) => {
                msg.channel_id.say(
                    &ctx.http,
                    format!(
                        "https://nekos.moe/image/{}",
                        nekos_client.get_image_from_buffer(nsfw).unwrap().id
                    ),
                )?;
            }
            Err(e) => {
                msg.channel_id.say(&ctx.http, format!("{:#?}", e))?;
                return Ok(());
            }
        }
    }

    Ok(())
}

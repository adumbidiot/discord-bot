use nekos::{
    Client,
    Image,
	NekosError
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
use std::sync::{
    Arc,
    RwLock,
};

type ShareVec = Arc<RwLock<Vec<Image>>>;

pub struct Nekos {
    opts: Arc<CommandOptions>,
    client: Client,
    nsfw_buffer: ShareVec,
    buffer: ShareVec,
}

impl Nekos {
    pub fn new() -> Self {
        let mut opts = CommandOptions::default();
        opts.desc = Some(String::from("Get a random catgirl image"));

        Nekos {
            opts: Arc::from(opts),
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
        self.get_buffer(nsfw).write().unwrap().pop()
    }

    fn populate_buffer(&self, nsfw:bool) -> Result<(), NekosError> {
        self.client.get_random_images(nsfw, 100).map(|image_list| {
            self.get_buffer(nsfw)
                .write()
                .unwrap()
                .extend_from_slice(&image_list.images)
        })
    }
}

impl Command for Nekos {
    fn execute(&self, _: &mut Context, msg: &Message, mut args: Args) -> Result<(), CommandError> {
        let nsfw = args
            .single::<String>()
            .map(|s| s == "--nsfw")
            .unwrap_or(false);

        if let Some(image) = self.get_image_from_buffer(nsfw) {
            msg.channel_id
                .say(format!("https://nekos.moe/image/{}", image.id))?;
        } else {
            match self.populate_buffer(nsfw) {
                Ok(()) => {
                    msg.channel_id.say(format!(
                        "https://nekos.moe/image/{}",
                        self.get_image_from_buffer(nsfw).unwrap().id
                    ))?;
                }
                Err(e) => {
                    msg.channel_id.say(format!("{:#?}", e))?;
                    return Ok(());
                }
            }
        }

        Ok(())
    }

    fn options(&self) -> Arc<CommandOptions> {
        self.opts.clone()
    }
}

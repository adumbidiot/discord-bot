mod commands;

use schoology::client::Client as SchoologyClient;
use serde::Deserialize;
use serenity::{
    client::{
        Client,
        Context,
        EventHandler,
    },
    framework::standard::{
        help_commands,
        StandardFramework,
    },
    model::gateway::Ready,
};
use std::{
    collections::HashMap,
    path::Path,
    sync::Arc,
};
use toml::Value;

struct Handler;

impl EventHandler for Handler {
    // TODO: Add logging somehow?
    fn ready(&self, _ctx: Context, _ready: Ready) {
        println!("[INFO] Logged in");
    }
}

#[derive(Deserialize, Debug)]
struct Config {
    // TODO: Validate function
    token: String,
    schoology: SchoologyConfig,
    fml: FmlConfig,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Deserialize, Debug)]
struct SchoologyConfig {
    token: String,
    secret: String,
}

#[derive(Deserialize, Debug)]
struct FmlConfig {
    key: String,
}

fn load_config(p: &Path) -> Option<Config> {
    // TODO: Result
    if !p.exists() {
        println!("[ERROR] {} does not exist", p.display());
        return None;
    }

    let data = std::fs::read(p)
        .map_err(|_| println!("[ERROR] {} could not be read", p.display()))
        .ok()?;
    toml::from_slice(&data).map_err(|err| dbg!(err)).ok()
}

fn main() {
    println!("[INFO] Loading config.toml...");
    let config = load_config(Path::new("./config.toml")).expect("Could not load config.toml"); // TODO: Move to seperate module?
    let mut client = Client::new(&config.token, Handler).expect("Error creating client");
    let schoology_client = Arc::from(SchoologyClient::new(
        // TODO: Check if token exists (has_schoology()), then create/add conditonally
        config.schoology.token,
        config.schoology.secret,
    ));

    let fml_client = Arc::from(fml::Client::new(&config.fml.key));

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~"))
        .cmd("ping", commands::Ping::new())
        .cmd("zalgoify", commands::Zalgoify::new())
        .cmd("vaporwave", commands::Vaporwave::new())
        .cmd("xkcd", commands::Xkcd::new())
        .cmd("fml", commands::Fml::new(fml_client.clone())) // TODO: Finish formatting command output
        .cmd("ttt", commands::TicTacToe::new())
        .cmd("urban", commands::Urban::new())
        .cmd("nekos", commands::Nekos::new())
        .group("schoology", |g| {
            g.prefixes(vec!["schoology"])
                .desc("A group with commands accessing the schoology api")
                .cmd(
                    "group",
                    commands::SchoologyGroup::new(schoology_client.clone()),
                )
                .cmd(
                    "user",
                    commands::SchoologyUser::new(schoology_client.clone()),
                )
        })
        .help(help_commands::plain)
        .on_dispatch_error(|_, msg, error| {
            println!("[ERROR] {:?}{}", error, msg.content);
        });

    client.with_framework(framework);

    println!("[INFO] Logging in...");
    if let Err(why) = client.start() {
        // Does this use autosharding?
        println!("[ERROR] {:?}", why);
    }

    println!("[INFO] Shutting down...");
}

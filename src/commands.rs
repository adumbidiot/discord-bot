mod fml;
mod groups;
mod nekos;
mod ping;
// mod schoology;
// mod tic_tac_toe;
// mod urban;
mod xkcd;

pub use crate::commands::{
    fml::FML_COMMAND,
    nekos::{
        NekosClient,
        NEKOS_COMMAND,
    },
    ping::PING_COMMAND,
    xkcd::XKCD_COMMAND,
};

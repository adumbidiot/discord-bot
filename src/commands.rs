mod fml;
mod groups;
mod ping;
mod schoology;
mod tic_tac_toe;
mod urban;
mod vaporwave;
mod xkcd;
mod zalgoify;
pub use crate::commands::{
    fml::Fml,
    ping::Ping,
    schoology::{
        SchoologyGroup,
        SchoologyUser,
    },
    tic_tac_toe::TicTacToe,
    urban::Urban,
    vaporwave::Vaporwave,
    xkcd::Xkcd,
    zalgoify::Zalgoify,
};

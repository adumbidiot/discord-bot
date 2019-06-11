mod fml;
mod groups;
mod ping;
mod schoology;
mod tic_tac_toe;
mod vaporwave;
mod xkcd;
mod zalgoify;
pub use crate::commands::{
    fml::Fml,
    schoology::{
        SchoologyGroup,
        SchoologyUser,
    },
    tic_tac_toe::TicTacToe,
    xkcd::Xkcd,
};
pub use ping::Ping;
pub use vaporwave::Vaporwave;
pub use zalgoify::Zalgoify;

mod fml;
mod groups;
mod ping;
mod schoology;
mod vaporwave;
mod xkcd;
mod zalgoify;
pub use self::{
    fml::Fml,
    schoology::{
        SchoologyGroup,
        SchoologyUser,
    },
    xkcd::Xkcd,
};
pub use ping::Ping;
pub use vaporwave::Vaporwave;
pub use zalgoify::Zalgoify;

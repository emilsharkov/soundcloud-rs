mod album;
mod builder;
mod client;
mod playlists;
mod search;
mod tracks;
mod users;

#[cfg(test)]
mod tests;

pub use builder::ClientBuilder;
pub use client::*;

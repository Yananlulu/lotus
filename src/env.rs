#[cfg(not(debug_assertions))]
pub fn version() -> String {
    format!("{}({})", env!("GIT_HEAD"), env!("BUILD_TIME"))
}

#[cfg(debug_assertions)]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

pub const NAME: &'static str = env!("CARGO_PKG_NAME");
pub const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
pub const HOMEPAGE: &'static str = env!("CARGO_PKG_HOMEPAGE");
pub const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
pub const BANNER: &'static str = include_str!("banner.txt");

pub mod parsers;

pub mod web;
pub mod plots {
    #[cfg(target_family = "unix")]
    pub mod cli;
    pub mod web;
}

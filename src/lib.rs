pub mod parsers;

pub mod web;
pub mod plots {
    #[cfg(not(target_family = "wasm"))]
    pub mod cli;
    pub mod web;
}

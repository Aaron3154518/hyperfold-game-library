#![feature(trait_upcasting)]
#![feature(extract_if)]

mod init;

hyperfold_engine::game_crate!();

use hyperfold_engine::system_macro;

fn main() {
    hyperfold_engine::run::<_engine::SFoo>();
}

#![feature(trait_upcasting)]
#![feature(drain_filter)]

mod init;

hyperfold_engine::game_crate!();

fn main() {
    hyperfold_engine::run::<_engine::SFoo>();
}

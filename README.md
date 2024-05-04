### Installation

```
cargo init
git submodule add https://github.com/Aaron3154518/hyperfold-engine.git
printf "\n[dependencies.hyperfold_engine]\npath = \"./hyperfold-engine\"\nversion = \"0.1.0\"\ndependency = \"\"" >> Cargo.toml
echo "nightly" > rust-toolchain
```

Copy sdl into `./hyperfold-engine`

https://github.com/libsdl-org/SDL/releases  
https://github.com/libsdl-org/SDL_ttf/releases  
https://github.com/libsdl-org/SDL_image/releases

Copy dlls into `./`

`main.rs`:
```
#![feature(trait_upcasting)]
#![feature(extract_if)]

hyperfold_engine::game_crate!();

fn main() {
    hyperfold_engine::run::<_engine::SFoo>();
}
```

#### VSCode
```
mkdir .vscode
touch settings.json
```

`settings.json`:
```
{
    "rust-analyzer.linkedProjects": [
        "./Cargo.toml",
        "./hyperfold-engine/Cargo.toml"
    ],
    "rust-analyzer.showUnlinkedFileNotification": false
}
```

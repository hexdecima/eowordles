## eowordles
A browser Wordle-like game of guessing the random daily Terraria NPC.
Works nicely but still very much lacking some features.

Written entirely in Rust, with [Leptos](https://leptos.dev/).

## Developing
**If using [Nix](https://nixos.org/)**, just `nix develop`.
**If not**, install Rust (`1.88.0+` with `wasm32-unknown-unknown` target) and [Trunk](https://trunkrs.dev/)

Make changes to `./src` and watch changes by running `trunk run --open`.

## Features 
- [x] Show amount of guesses at the end. 
- [x] Show NPC sprites (note: not all animated yet).
- [x] Show yesterday's NPC.
- [ ] Show a rough estimate based on guesses.
- [ ] Remember guesses between refreshes.

## License
Unlicense.

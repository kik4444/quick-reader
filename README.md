# quick-reader
A program that enables its user to quickly read through text by encouraging the silencing of their subvocalization.

# Demo video 
https://github.com/kik4444/quick-reader/assets/7779637/571aae37-e977-40a3-9888-a217130e6b05

# Build instructions
- Install Rust https://www.rust-lang.org/tools/install
- Install trunk with `cargo install trunk`
- Install NPM https://nodejs.org/en/download
- Clone the repo and cd to it
- Install frontend dependencies with `npm install`
## Web
- Run `trunk build --release`
- The server assets will be in the `./dist` directory
## Tauri
- Follow Tauri prerequisites https://tauri.app/v1/guides/getting-started/prerequisites
- Run `npm run tauri build`
- The binary will be in `./target/release`

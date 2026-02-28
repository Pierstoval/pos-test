# POS prototype

## Installation

Requirements:

- Node.js version 22 or later
- PNPM version 10 or later
- Rust and Cargo, version 1.93 or later

To install:

- Check that you have the necessary pre-requisites for Tauri: https://tauri.app/start/prerequisites/
- Run the `pnpm install` command

## Development

Run the `pnpm run tauri dev` command to start working on the project.

## Build for release

Run the `pnpm run tauri build` command to build the project for production.

- The app as a simple executable file is in the `src-tauri/target/release/pos` file (or `pos.exe` on Windows)
- Several installers for your platform are located in the `src-tauri/target/release/bundle/` directory.

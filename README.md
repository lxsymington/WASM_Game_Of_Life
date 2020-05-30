# WASM Game Of Life

## What this is

This is the code I have written as a result of following the exercises in the Rust WASM book which can be found [here](https://rustwasm.github.io/docs/book/). As the purpose of this is learning how to use Rust and WASM to create a _browser consumable_ module, this has been the focus and the remainder has been made to _just about function_!

## Usage / Install

### Prerequisites

In order to run this project you will need to have installed:

- Rust
  - The [recommended way](https://www.rust-lang.org/tools/install) to do this is via rustup.
  - If you install this on WSL Rust expects a linker to be present on the system. WSL installations do not seem to always have one. On Ubuntu this can be resolved by installing `build-essential` with apt.
- WASMpack
  - The [WASMpack website has installation instructions](https://rustwasm.github.io/wasm-pack/).
- Node
  - There are various ways of installing node, [here](https://nodejs.org/en/download/package-manager/) are some of the package managers that offer it .
- npm / yarn
  - npm comes with Node. If you want to use yarn instructions can be found [here](https://classic.yarnpkg.com/en/docs/install).

### Setup

Once the prerequisites have been fulfilled. Clone this repository with the _`--recurse-submodules` flag_. This repository will produce a local node module. The submodule actually uses it!

In the root of _this_ repository run `wasm-pack build` this will build the `wasm-game-of-life` module that the front-end will use.

Then navigate to the root of the submodule (`./www`) and run `npm i`.

Finally run `npm run start` this should start a dev server running on `localhost:8080`.

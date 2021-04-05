# How to WASM with Rust?

## Intitialize Working Directory
mkdir wasm-tutorial
cd wasm-tutorial

## Initialize git (optional)
git init .

## Make folder for Rust project
mkdir rust

## Initialize npm & install Webpack in root directory
npm init -y
npm install --save-dev webpack webpack-cli
npm install --save-dev @webpack-cli/generators
npx webpack init ./ --force --template=default

## Add .gitignore
touch .gitignore

## Test configuration (and open http://localhost:8080/)
npm run serve

## Initialize rust
cd rust
cargo init .

### and test
cargo run

## Webpack + Rust
npm install --save-dev rimraf
npm install --save-dev @wasm-tool/wasm-pack-plugin

## WebWorker
npm install --save comlink

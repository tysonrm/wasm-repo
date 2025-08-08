# Writing **TAHO** Models in Javascript

> 📝 Nota Bene: This procedure assumes you have rust, wasmtime, wasmedge and nodejs installed. If not, the first step is to run the taho install script, which is coming soon...

## Quick Start

1. Clone this repo
   ```sh
   git clone https://github.com/opnbook/taho.git
   ```
2. Install NPM in `models/polyglot/js` directory.
   ```sh
   npm i
   ```
3. Install JCO.
   ```sh
   npm i @bytecodealliance/jco --save-dev
   ```
4. Run the NPM script `compile`.
   ```sh
   npm run compile
   ```
5. From the `crates/repo` directory:
   ```sh
   cargo run
   ```
6. From the `host` directory:
   ```sh
   cargo run
   ```
   
## Development Steps

1. To generate ts types from the wit:
   ```sh
   witbindgen ../../wit
   ```
2. To use intellisense to autofill, change model.js to model.ts and import the generated types. Implement the interfaces and extend the classes, allowing intellisense to fill in the missing code.
3. Change the file extension back to .js.
4. Change all the `class`es to `const`ants and remove the `implement`s and `extend`s.
5. Remove the `import` statements.
6. To compile and run follow steps 4-6 in the first section above.

> 📝 Nota Bene: Alternatively, you can start with ts and compile to js, but you still need to change the `class`es to `const`s and remove references to the generated types. The wasm compiler only understands `const`s. This goes for items you aren't even exporting!



# Rust with WASM

## Install
New WASM App
`cargo generate --git https://github.com/rustwasm/wasm-pack-template`

Install Wasm pack
```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh`
rustup target add wasm32-unknown-unknown ]
# unknown-unknown = “Compile on almost any machine, run on almost any machine”
cargo install wasm-pack
```

Build
`wasm-pack build`
or
`wasm-pack build --target web`


## Attributes
`#[]` syntax, i.e. `#[wasm_bindgen]`
- Control how the Rust compiler behaves (e.g., #[inline] to suggest that a function should be inlined).
- Indicate conditional compilation (e.g., #[cfg(target_os = "windows")] for Windows-specific code).
- Manage crates and module inclusion (e.g., #[macro_use]).
- Document the code (e.g., /// for comments that become documentation, or #[doc(hidden)] to hide items from documentation).

## Generics
`<N>` in Rust is part of the language's generics system, allowing for type-safe, reusable code that works with multiple types, while any in TypeScript is a type that bypasses the TypeScript static type checking, allowing JavaScript values of any type to be used. They serve different purposes in their respective languages and have different impacts on type safety and code behavior.

## Unique or More Pronounced in Rust:

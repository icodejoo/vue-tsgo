# vue-tsgo — Zed extension

A minimal [Zed](https://zed.dev) extension that uses **vue-tsgo** as the
language server for `.vue` files. Structure adapted from the official
[zed-extensions/vue](https://github.com/zed-extensions/vue), with the
Volar/Node language server replaced by the native vue-tsgo binary
(`tsgo --lsp --stdio`).

## Layout

```
editors/zed/
  extension.toml          # declares the Vue.js language + grammar + vue-tsgo LSP
  Cargo.toml / src/lib.rs  # Rust → WASM; launches the vue-tsgo binary
  languages/vue/*          # language config + tree-sitter queries
  bin/tsgo[.exe]           # the vue-tsgo binary (NOT committed — see below)
```

## Build & install (dev extension)

Prerequisites: Rust with the `wasm32-wasip1` target
(`rustup target add wasm32-wasip1`), plus the Go toolchain to build the binary.

1. Build the vue-tsgo binary and place it in `bin/`:

   ```bash
   # from the repo root
   make build-binary
   mkdir -p editors/zed/bin
   cp golar/tsgo editors/zed/bin/tsgo            # or tsgo.exe on Windows
   ```

   (Alternatively, point the extension at a binary elsewhere via the
   `lsp.vue-tsgo.binary.path` setting — see below.)

2. In Zed: **uninstall the official "Vue" extension** first (both define the
   `Vue.js` language, which would conflict).

3. Command palette → **`zed: install dev extension`** → select this
   `editors/zed` directory. Zed compiles the Rust to WASM and fetches the
   grammar.

4. Open a `.vue` file. Highlighting comes from the grammar; diagnostics,
   hover, go-to-definition, and completions come from vue-tsgo.

## Settings

Point at an explicit binary (recommended if you don't bundle one in `bin/`):

```jsonc
"lsp": {
  "vue-tsgo": {
    "binary": { "path": "/absolute/path/to/tsgo" }
  }
}
```

And select vue-tsgo for `.vue`:

```jsonc
"languages": {
  "Vue.js": { "language_servers": ["vue-tsgo", "..."] }
}
```

## Notes

- The binary is platform-specific; build it for each OS/arch you use.
- This is a local dev extension; it is not published to the Zed registry.

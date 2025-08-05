```sh
bun build ./core/build/dev/javascript/core/core/core.mjs --outfile core.js --minify-syntax --minify-whitespace
cargo run check_mjs
```
delete the export block at the end
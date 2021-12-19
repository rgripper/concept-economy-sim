## Building for web:

Something thats temporarily needed until Bevy 0.6

```
cargo update -p tracing-wasm --precise 0.2.0
```

```
wasm-pack build --target web --release
```

## Starting (if you have NodeJS installed)

```
npx serve .
```

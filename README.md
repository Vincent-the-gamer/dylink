# Dylink

Create dynamic link libraries(.dll/.dylib/.so) in Rust, for Node.js use.

## Usage

Download dylib file(.dll/.dylib/.so) from release, then call function use `ffi-rs` or etc tools.

```ts
import { close, DataType, load, open } from "ffi-rs"

const library = "libdylink"
const path = "/path/to/.dylib or .so or .dll"

open({
    library,
    path
})

const send = load({
    library,
    funcName: "send_notification",
    retType: DataType.Void,
    paramsType: [DataType.String, DataType.String],
    paramsValue: ["Ciallo from Node.js", "Ah! senpai~ ciallo~"]
})

close(library)
```

## Build

### Locally

Ensure you have Rust installed, then

Run `cargo build --release`

### Cross Platform

Ensure you have Rust and Docker, then

```shell
cargo install cross --git https://github.com/cross-rs/cross

# use cross
cross build --release --target x86_64-pc-windows-gnu
```

Targets: 
- x86_64-unknown-linux-gnu
- x86_64-pc-windows-gnu


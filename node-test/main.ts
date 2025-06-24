import { close, DataType, load, open } from "ffi-rs"

const library = "libdylink"
const path = "../target/release/libdylink.dylib"

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


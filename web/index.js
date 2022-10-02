import init, { greet } from "webassembly_rust";

init().then(() => {
    greet("WebAssembly");
    });
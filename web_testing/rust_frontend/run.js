import init, { rust_main_entry } from "./pkg/rust_frontend.js";

async function run() {
  await init();
  rust_main_entry();
}

run();

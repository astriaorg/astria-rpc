use std::process::{exit, Command};

fn main() {
    println!("cargo:rerun-if-changed=proto/");

    // Run the `buf generate` command to generate the Rust files
    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR env var must be set by cargo");
    let status = Command::new("buf")
        .arg("generate")
        .arg("--output")
        .arg(out_dir)
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .status()
        .expect("failed to generate protobuf bindings; is `buf` installed?");

    if !status.success() {
        exit(status.code().unwrap_or(-1));
    }
}

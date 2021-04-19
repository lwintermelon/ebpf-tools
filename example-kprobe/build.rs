fn main() {
    #[cfg(feature = "user")]
    build_bpf()
}

#[cfg(feature = "user")]
fn build_bpf() {
    use std::{env, process::Command};

    let target_dir = env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "../target".to_string());
    let target_dir = format!("{}/bpf", target_dir);

    let args = &[
        "+nightly-2020-12-31",
        "rustc",
        "--package=example-kprobe",
        "--bin=example-kern",
        "--features=kern",
        "--no-default-features",
        "--",
        "-Clinker-plugin-lto",
        "-Clinker-flavor=wasm-ld",
        "-Clinker=bpf-linker",
        "-Clink-arg=--target=bpf",
    ];
    let output = Command::new("cargo")
        .env("CARGO_TARGET_DIR", &target_dir)
        .args(args)
        .output()
        .expect("failed to build bpf code");
    if !output.status.success() {
        let error = String::from_utf8(output.stderr).expect("malformed error message");
        panic!("{}", error);
    }
    Command::new("sed")
        .current_dir(&target_dir)
        .arg("-i")
        .arg("s/ty__/type/g")
        .arg("debug/example-kern")
        .output()
        .expect("failed to patch bpf object");

    println!("cargo:rustc-env=BPF_CODE={}/debug/example-kern", target_dir);
}

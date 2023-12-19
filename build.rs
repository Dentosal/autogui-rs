#![allow(dead_code)]

use std::{env, fs, path::PathBuf, process::Command};

fn link_helpers_macos() {
    println!("cargo:rerun-if-changed=platform_helpers/macos/Sources/Keycode/Keycode.swift");

    let swift_build_path = PathBuf::from("platform_helpers/macos/.build/");

    let profile = env::var("PROFILE").unwrap();
    let root_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    // Compile the swift library
    let status = Command::new("swift")
        .args(&[
            "build",
            "--package-path",
            "platform_helpers/macos/",
            "-c",
            &profile,
        ])
        .status()
        .unwrap();

    assert!(status.success());

    let mut from_path = PathBuf::from(&swift_build_path);
    from_path.push(&profile);
    from_path.push("libKeycode.a");

    let mut to_path = PathBuf::from(root_dir);
    to_path.push("target");
    to_path.push("libkeycode.a");

    fs::rename(from_path, to_path).expect("Unable to move libKeycode.a");
    fs::remove_dir_all(swift_build_path).expect("Unable to clean up");

    // Add linker args
    let target_info = Command::new("swift")
        .arg("-print-target-info")
        .output()
        .unwrap()
        .stdout;
    let swift_env: serde_json::Value = serde_json::from_slice(&target_info).unwrap();
    for path in swift_env["paths"]["runtimeLibraryPaths"]
        .as_array()
        .unwrap()
    {
        println!("cargo:rustc-link-search=native={}", path.as_str().unwrap());
    }

    println!("cargo:rustc-link-search=native=target");
    println!("cargo:rustc-link-lib=static=keycode");
}

fn codegen_keycodes_macos() {
    let contents = fs::read("/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/System/Library/Frameworks/Carbon.framework/Versions/A/Frameworks/HIToolbox.framework/Versions/A/Headers/Events.h").expect("missing Events.h");
    let contents = String::from_utf8_lossy(&contents);
    let magic = "/* keycodes for keys that are independent of keyboard layout*/\nenum {\n";
    let start = magic.len() + contents.find(magic).expect("Missing magic comment");
    let end = contents[start..].find("}").expect("Enum not closed");
    let mut result = String::from("fn get_fixed_keycode(name: &str) -> Option<u16> {\n");
    result.push_str("match name {\n");
    for line in contents[start..][..end].split('\n') {
        let Some((l, r)) = line.split_once("=") else {
            continue;
        };
        let l = l.trim().strip_prefix("kVK_").expect("Missing kVK_ prefix");
        let r = r.trim().trim_end_matches(",");
        result.push_str(&format!("{l:?} => Some({r}),\n"));
    }
    result.push_str("_ => None,\n}}\n");

    let mut path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    path.push("target");
    path.push("macos_fixed_keycodes.rs");
    fs::write(path, result).expect("Unable to write file");
}

fn main() {
    #[cfg(target_os = "macos")]
    link_helpers_macos();
    #[cfg(target_os = "macos")]
    codegen_keycodes_macos();
}

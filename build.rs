use std::process::Command;

fn link_helpers_macos() {
    let status = Command::new("clang").args(&[
        "platform_helpers/macos/keycode.m",
        "-fdiagnostics-color",
        "-Werror", "-fobjc-arc", "-fmodules",
        "-mmacosx-version-min=10.10",
        "-c", "-o", "target/keycode.o"
    ]).status().unwrap();

    assert!(status.success());

    let status = Command::new("ar").args(&[
        "crus", "target/libkeycode.a", "target/keycode.o"
    ]).status().unwrap();

    assert!(status.success());

    println!("cargo:rustc-link-search=native=target");
    println!("cargo:rustc-link-lib=static=keycode");
}

fn main() {
    #[cfg(target_os = "macos")]
    link_helpers_macos();
}

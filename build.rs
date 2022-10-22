
use std::process::Command;
use std::env;

fn main() {
    println!("cargo:rustc-link-search=../SDL2");
    let out = env::var("PROFILE").expect("PROFILE not specified");
    Command::new("cp")
        .args(["../SDL2/SDL2.dll", &format!("target/{}", out)])
        .output()
        .expect("SDL2 copy failed");

    Command::new("cp")
        .args(["../SDL2/SDL2_ttf.dll", &format!("target/{}", out)])
        .output()
        .expect("SDL2_ttf copy failed");
}
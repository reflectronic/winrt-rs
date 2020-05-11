use std::process::Command;
use std::env;

fn main() {
    let out = env::var("OUT_DIR").unwrap();

    Command::new(r"ilasm")
        .args(&["/DLL"])
        .arg(format!("/output={}/RustWinRT.Tests.winmd", out).as_str())
        .arg("./tests/RustWinRT.Tests.il")
        .status()
        .unwrap();
    
    println!("cargo:rerun-if-changed=./tests/RustWinRT.Tests.il");
}
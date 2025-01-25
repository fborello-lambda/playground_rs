use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Define paths
    let out_dir = env::var("OUT_DIR").unwrap();
    let guest_s_path = PathBuf::from("src/guest.s");
    let guest_o_path = PathBuf::from(out_dir.clone()).join("guest.o");
    let guest_bin_path = PathBuf::from(out_dir.clone()).join("guest");

    // Assemble the assembly file
    Command::new("as")
        .args([
            "-32",
            guest_s_path.to_str().unwrap(),
            "-o",
            guest_o_path.to_str().unwrap(),
        ])
        .status()
        .expect("Failed to assemble guest.s");

    // Link the object file into a binary
    Command::new("ld")
        .args([
            "-m",
            "elf_i386",
            "--oformat",
            "binary",
            "-N",
            "-e",
            "_start",
            "-Ttext",
            "0x10000",
            "-o",
            guest_bin_path.to_str().unwrap(),
            guest_o_path.to_str().unwrap(),
        ])
        .status()
        .expect("Failed to link guest.o");

    // Inform Cargo about the output binary
    println!("cargo:rustc-link-search=native={}", out_dir);
}

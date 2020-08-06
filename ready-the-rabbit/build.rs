use std::process::Command;

fn main() {
  Command::new("cargo")
    .args(&["bpf", "build"])
    .current_dir("../block-the-rabbit")
    .status()
    .unwrap();

  println!("cargo:rerun-if-changed=../block-the-rabbit/target/bpf/programs/block-the-rabbit/block-the-rabbit.elf");
  println!("cargo:rerun-if-changed=../block-the-rabbit/src/block.rs");
}

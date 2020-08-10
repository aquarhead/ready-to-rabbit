use std::process::Command;

fn main() {
  Command::new("cargo").args(&["bpf", "build", "block"]).status().unwrap();

  println!("cargo:rerun-if-changed=xdp/block.rs");
}

use anyhow::Result;
use log::error;
use redbpf::Module;
use serde::{Deserialize, Serialize};
use std::{process::Command, thread, time};

#[derive(Debug, Serialize, Deserialize)]
struct ClusterStatus {
  running_nodes: Vec<String>,
}

fn main() -> Result<()> {
  simple_logger::init().expect("error initializing logger");

  let prog = include_bytes!("../target/bpf/programs/block/block.elf");
  let mut module = Module::parse(prog).expect("error parsing BPF code");

  for program in module.programs.iter_mut() {
    program
      .load(module.version, module.license.clone())
      .expect("failed to load program");
  }

  for prog in module.xdps_mut() {
    let interfaces = std::fs::read_dir("/sys/class/net").expect("failed to list interfaces");
    for interface in interfaces {
      if let Ok(iface) = interface {
        if let Ok(ifa) = iface.file_name().into_string() {
          prog
            .attach_xdp(&ifa, Default::default())
            .expect("failed to attach XDP program");
        }
      }
    }
  }

  loop {
    let wait = time::Duration::from_millis(100);
    thread::sleep(wait);

    match Command::new("rabbitmqctl")
      .args(&["cluster_status", "--quiet", "--formatter", "json"])
      .output()
    {
      Ok(output) => {
        if output.status.success() {
          if let Ok(cs_json) = String::from_utf8(output.stdout) {
            match serde_json::from_str::<ClusterStatus>(cs_json.as_str()) {
              Ok(cs) => {
                if cs.running_nodes.len() > 1 {
                  break;
                }
              }
              Err(error) => error!("failed to parse cluster_status json: {}", error),
            }
          }
        }
      }
      Err(error) => error!("failed to get cluster_status: {}", error),
    }
  }

  Ok(())
}

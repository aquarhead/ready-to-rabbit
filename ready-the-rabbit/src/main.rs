use anyhow::Result;
use log::error;
use redbpf::load::Loader;
use serde::{Deserialize, Serialize};
use std::{process::Command, thread, time};

#[derive(Debug, Serialize, Deserialize)]
struct ClusterStatus {
  running_nodes: Vec<String>,
}

fn main() -> Result<()> {
  simple_logger::init().expect("error initializing logger");

  let prog = include_bytes!("../../block-the-rabbit/target/bpf/programs/block-the-rabbit/block-the-rabbit.elf");
  let mut loader = Loader::load(prog).expect("error loading XDP program");

  for prog in loader.module.xdps_mut() {
    // Ideally we should not hardcode the interface, but there's a bug in RedBPF currently
    //
    // let interfaces = std::fs::read_dir("/sys/class/net").expect("failed to list interfaces");
    // for interface in interfaces {
    //   if let Ok(iface) = interface {
    //     if let Ok(ifa) = iface.file_name().into_string() {
    //       prog
    //         .attach_xdp(&ifa, xdp::Flags::default())
    //         .expect("failed to attach XDP program");
    //     }
    //   }
    // }
    prog
      .attach_xdp("eth0", Default::default())
      .expect("failed to attach XDP program");
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

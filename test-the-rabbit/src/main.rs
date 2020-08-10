use anyhow::{bail, Result};
use lapin::Connection;
use rand::{thread_rng, Rng};
use std::{env, thread, time::Duration};

fn main() -> Result<()> {
  if let Some(uri) = env::args().skip(1).next() {
    for _ in 0..20 {
      let uuu = uri.clone();
      thread::spawn(move || {
        smol::run(async {
          let mut rng = thread_rng();
          loop {
            if let Ok(conn) = Connection::connect(&uuu, Default::default()).await {
              if let Ok(_) = conn.create_channel().await {
                let wait: u32 = rng.gen_range(500, 1000);
                thread::sleep(Duration::from_millis(wait.into()));
                let _ = conn.close(200, "test").await;
              } else {
                println!("failed to create channel");
              }
            } else {
              println!("failed to connect");
            }

            let wait: u32 = rng.gen_range(500, 1000);
            thread::sleep(Duration::from_millis(wait.into()));
          }
        });
      });
    }

    loop {}
  }

  bail!("requires AMQP URI at 1st argument");
}

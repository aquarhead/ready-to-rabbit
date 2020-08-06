#![no_std]
#![no_main]
program!(0xFFFFFFFE, "GPL");

use redbpf_probes::xdp::prelude::*;

#[xdp("block")]
pub fn block(ctx: XdpContext) -> XdpResult {
  if let Ok(transport) = ctx.transport() {
    if transport.dest() == 5672 {
      return Ok(XdpAction::Drop);
    }
  }

  Ok(XdpAction::Pass)
}

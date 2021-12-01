use anchor_lang::prelude::*;

declare_id!("FncsSwfyvedwjezBW2Ss4ryCrNPFif71p5DJRo2LXZg6");

#[program]
pub mod myepicproject {
  use super::*;
  pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
    Ok(())
  }
}

#[derive(Accounts)]
pub struct StartStuffOff {}
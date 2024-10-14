
use anchor_lang::prelude::*;

declare_id!("8F9ebQpQxcQHA8oo4wvFHSEvVEk3CCtaqi7YqBF4A66e");

#[program]
pub mod coin_toss {
    use super::*;

    pub fn play(ctx: Context<Play>, guess: u8) -> Result<()> {
         // Coin flip result: 0 or 1
        let result = Clock::get()?.unix_timestamp % 2; 
      // 0 (Heads) or 1 (Tails)
        let player_guess = guess % 2; 

        if result == player_guess as i64 {
            msg!("You guessed correctly.");
        } else {
            msg!("You guessed incorrectly.");
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Play<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

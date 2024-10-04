// use anchor_lang::prelude::*;

// declare_id!("97otWVsih7AnELD1BsZsrzgmNq1PkqGFp2iJbG2XapYd");

// #[program]
// pub mod solana_blink {
//     use super::*;

//     pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
//         Ok(())
//     }
// }

// #[derive(Accounts)]
// pub struct Initialize {}

use anchor_lang::prelude::*;

declare_id!("97otWVsih7AnELD1BsZsrzgmNq1PkqGFp2iJbG2XapYd");


#[program]
pub mod solana_blink {
    use super::*;

    pub fn count_transactions(
        ctx: Context<Blink>,
        amount: u64
    ) -> Result<u64> {
        let sender = ctx.accounts.sender.key;
        let receiver = ctx.accounts.receiver.key;
        
        msg!("Counting transactions from {:?} to {:?}", sender, receiver);

        // Dummy implementation for now. Replace with actual transaction counting.
        if amount == 0 {
            msg!("Counting all transactions...");
            return Ok(10);
        } else {
            msg!("Counting transactions with amount {:?}", amount);
            return Ok(5);
        }
    }
}

#[derive(Accounts)]
pub struct Blink<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    #[account(mut)]
    pub receiver: AccountInfo<'info>,
}

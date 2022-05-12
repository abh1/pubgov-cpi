use anchor_lang::prelude::*;
use anchor_spl::token::{Transfer, Mint, Token, TokenAccount};
use anchor_spl::token::accessor::amount;
use anchor_spl::associated_token::get_associated_token_address;
use solana_program::{instruction::Instruction, program::invoke_signed};

declare_id!("7jcZwQs9pu6vru4cy1EjQyZseCE2NW29eT2vy6NRzeCo");


const NEWSTOKEN: Pubkey = solana_program::pubkey!("3qq7ExpwRRAAexGNpUVoFkiTfSB1uo8ezsbyAoxhyryo");
const TREASURY: Pubkey = solana_program::pubkey!("6kgSK2hFDjUCS3wafYYW2VSwkjETuqHdByWddwmytyp7");
const TOKEN_PROGRAM: Pubkey = solana_program::pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

const FEE_AMOUNT:u64 = 100000000;

#[program]
pub mod newsreport {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>)  -> ProgramResult  {
        let report_acc = &mut ctx.accounts.report_account;
        report_acc.authority = *ctx.accounts.authority.key;
        report_acc.status = 0;
        Ok(())
    }

    pub fn update_report(ctx: Context<UpdateReport>, report_uri: String) -> ProgramResult {
//        let newstoken = ctx.accounts.newstoken.key;
        let treasury = ctx.accounts.treasury.owner;
        if treasury == TREASURY {
            let report_acc = &mut ctx.accounts.report_account;
            report_acc.uri = report_uri.to_string();
            report_acc.status = 0;
            anchor_spl::token::transfer(
                CpiContext::new(
                    ctx.accounts.newstoken.to_account_info(),
                    anchor_spl::token::Transfer {
                        from: ctx.accounts.from.to_account_info(),
                        to: ctx.accounts.treasury.to_account_info(),
                        authority: ctx.accounts.authority.to_account_info(),
                        },
                    ),
                    FEE_AMOUNT,
                )?;   
        }
        Ok(())
    }
    pub fn push_for_vote(ctx: Context<PushforVote>) -> ProgramResult {
//        let newstoken = ctx.accounts.newstoken.key;
            let treasury = ctx.accounts.treasury.owner;
            if treasury == TREASURY {
                let report_acc = &mut ctx.accounts.report_account;
                report_acc.status = 1;
                anchor_spl::token::transfer(
                    CpiContext::new(
                        ctx.accounts.newstoken.to_account_info(),
                        anchor_spl::token::Transfer {
                            from: ctx.accounts.from.to_account_info(),
                            to: ctx.accounts.treasury.to_account_info(),
                            authority: ctx.accounts.authority.to_account_info(),
                            },
                        ),
                        FEE_AMOUNT,
                    )?;   
            }
        Ok(())
    }
      pub fn publish(ctx: Context<Publish>) -> ProgramResult {
 
                 Ok(())
             }
}


#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 566)]
    pub report_account: Account<'info, ReportAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub newstoken: Program<'info, Token>,
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub treasury: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateReport<'info> {
    #[account(mut, has_one = authority)]
    pub report_account: Account<'info, ReportAccount>,
    pub authority: Signer<'info>,
    pub newstoken: Program<'info, Token>,
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub treasury: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PushforVote<'info> {
    #[account(mut, has_one = authority)]
    pub report_account: Account<'info, ReportAccount>,
    pub authority: Signer<'info>,
    pub newstoken: Program<'info, Token>,
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub treasury: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Publish<'info> {
    #[account(mut)]
    pub report_account: Account<'info, ReportAccount>,
    
    pub governance: Signer<'info>,

}
 
#[account]
pub struct ReportAccount {
    pub uri: String,
    pub authority: Pubkey,
    pub status: u8,
}
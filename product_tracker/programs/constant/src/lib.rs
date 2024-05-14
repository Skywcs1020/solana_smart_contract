use anchor_lang::prelude::*;

pub mod constant;
pub mod error;
pub mod states;

use crate::{constant::*, error::TrackerError, states::*};

// Program address
declare_id!("HC7v6BhVukUcnzEqqkrv89ECrWemx1UTeMkmZchbs9AP");

#[program]
pub mod product_tracker {
    use super::*;

    pub fn init_user(ctx: Context<InitUser>, certificate: String, role: String) -> Result<()> {
        //Logic
        let user_account = &mut ctx.accounts.user_account;
        let authority = &mut ctx.accounts.authority;

        user_account.address = *authority.key;
        user_account.role = role;
        user_account.product_id = 0;
        user_account.certificate = certificate;

        Ok(())
    }

    pub fn create_product(ctx: Context<InitProduct>, product_name: String) -> Result<()> {
        let product_account = &mut ctx.accounts.product_account;
        let user_account = &mut ctx.accounts.user_account;

        product_account.record = Vec::new();
        product_account.record_count = 0;
        product_account.product_name = product_name;
        product_account.product_origin = user_account.address;

        user_account.product_id += 1;

        msg!("product address: {}", product_account.to_account_info().key);
        Ok(())
    }

    pub fn add_record(
        ctx: Context<AddRecord>,
        location: String,
        next_owner: Pubkey,
        certificate: String,
    ) -> Result<()> {
        let product_account = &mut ctx.accounts.product_account;
        let user_account = &mut ctx.accounts.user_account;
        let authority = &mut ctx.accounts.authority;
        let role = &mut user_account.role;
        let record_count = product_account.record_count as usize;

        if record_count == 0 {
            if &product_account.product_origin == authority.key {
                product_account.record.push(Record {
                    location,
                    next_owner,
                    certificate,
                    role: role.to_string(),
                });
                product_account.record_count += 1;
                Ok(())
            } else {
                return Err(TrackerError::Unauthorized.into());
            }
        } else {
            if record_count < 10 {
                if let Some(record) = product_account.record.get(record_count - 1) {
                    if &record.next_owner == authority.key {
                        product_account.record.push(Record {
                            location,
                            next_owner,
                            certificate,
                            role: role.to_string(),
                        });
                        msg!("5");
                        product_account.record_count += 1;
                        Ok(())
                    } else {
                        return Err(TrackerError::Unauthorized.into());
                    }
                } else {
                    return Err(TrackerError::Overflowed.into());
                }
            } else {
                return Err(TrackerError::Overflowed.into());
            }
        }
    }
}

#[derive(Accounts)]
#[instruction()]

pub struct InitUser<'info> {
    #[account(
        init,
        seeds = [USER_SEED, authority.key.as_ref()],
        bump,
        payer = authority,
        space = 2569 + 8
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction()]
pub struct InitProduct<'info> {
    #[account(
        init,
        seeds = [PRODUCT_SEED, authority.key.as_ref(), &[user_account.product_id as u8].as_ref()],
        bump,
        payer = authority,
        space = 10240,
    )]
    pub product_account: Account<'info, ProductAccount>,

    #[account(
        mut,
        seeds = [USER_SEED, authority.key.as_ref()],
        bump,
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction()]
pub struct AddRecord<'info> {
    #[account(
        mut,
        seeds = [USER_SEED, authority.key.as_ref()],
        bump,
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(mut,)]
    pub product_account: Account<'info, ProductAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

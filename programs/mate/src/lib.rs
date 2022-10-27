use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke, system_instruction},
};

declare_id!("3gzDR5dYBfKF21QZSWMiCR7bMar9o9hYV13pQ5co16xo");

#[program]
pub mod mate {
    use super::*;
    
    pub fn create_group(
        ctx: Context<CreateGroup>,
        name: String,
        ratio: u16,
        members: Vec<Pubkey>,
    ) -> Result<()> {
        let group = &mut ctx.accounts.group;
        group.name = (*name).to_string();
        group.ratio = ratio;
        group.members = members;
        group.treasury = *ctx.accounts.treasury.key;
        group.bump = *ctx.bumps.get("group").unwrap();

        emit!(GroupChanged {
            name
        });

        Ok(())
    }

    pub fn create_project_pda(
        ctx: Context<CreateProject>,
        name: String,
        group: String,
        project_type: String,
        ratio: u16,
        payments: Vec<Payment>,
        currency: String,
        amount: u64,
        start_date: u64,
        end_date: u64,
        client: Pubkey,
    ) -> Result<()> {
        let project = &mut ctx.accounts.project;
        project.name = (*name).to_string();
        project.group = group;
        project.project_type = project_type;
        project.ratio = ratio;
        project.treasury = *ctx.accounts.treasury.key;
        project.payments = payments;
        project.currency = currency;
        project.status = "INITIALIZATED".to_string();
        project.amount = amount;
        project.start_date = start_date;
        project.end_date = end_date;
        project.client = client;

        Ok(())
    }

    pub fn pay_project(ctx: Context<PayProject>) -> Result<()> {
        let project = &ctx.accounts.project;
        let members = [
            &mut ctx.accounts.member_0,
            &mut ctx.accounts.member_1,
            &mut ctx.accounts.member_2,
            &mut ctx.accounts.member_3,
            &mut ctx.accounts.member_4,
            &mut ctx.accounts.member_5,
            &mut ctx.accounts.member_6,
            &mut ctx.accounts.member_7,
            &mut ctx.accounts.member_8,
            &mut ctx.accounts.member_9,
        ];
        ctx.accounts.project.payments.iter().for_each(|payment| {
            let found = members
                .iter()
                .find(|account| account.key == &payment.member);
            match found {
                Some(member) => {
                    msg!("Paying {:#?} Lamports to {:#?}", payment.amount ,payment.member);
                    invoke(
                    &system_instruction::transfer(
                        ctx.accounts.payer.key,
                        &payment.member,
                        payment.amount,
                    ),
                    &[
                        ctx.accounts.payer.to_account_info().clone(),
                        member.to_account_info().clone(),
                    ],
                )},
                None => Ok(()),
            };
        });
        let project = &mut ctx.accounts.project;
        project.status = "PAYED".to_string();

        msg!("Project {:#?} Payed!", project.name);

        Ok(())
    }

}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateGroup<'info> {
    #[account(
        init,
        payer = payer,
        space = 9000,
        seeds = [b"group".as_ref(), name.as_ref()],
        bump
    )]
    pub group: Account<'info, Group>,
    /// CHECK:
    pub treasury: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub payer: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name: String, group: String)]
pub struct CreateProject<'info> {
    #[account(
        init,
        payer = payer,
        space = 9000,
        seeds = [b"project".as_ref(), name.as_ref(), group.as_ref()],
        bump
    )]
    pub project: Account<'info, Project>,
    /// CHECK:
    pub treasury: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub payer: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PayProject<'info> {
    #[account(mut)]
    pub project: Account<'info, Project>,
    /// CHECK:
    #[account(mut)]
    pub payer: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    /// CHECK:
    #[account(mut)]
    pub member_0: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub member_1: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub member_2: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub member_3: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub member_4: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub member_5: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub member_6: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub member_7: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub member_8: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub member_9: AccountInfo<'info>,
}

#[account]
pub struct Group {
    pub name: String,
    pub treasury: Pubkey,
    pub ratio: u16,
    pub members: Vec<Pubkey>,
    pub bump: u8,
}

#[account]
pub struct Project {
    pub name: String,
    pub group: String,
    pub project_type: String,
    pub treasury: Pubkey,
    pub ratio: u16,
    pub payments: Vec<Payment>,
    pub currency: String,
    pub status: String,
    pub amount: u64,
    pub common_expenses: u64,
    pub start_date: u64,
    pub end_date: u64,
    pub client: Pubkey,
    pub milestones: u8,
    pub bump: u8,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Payment {
    member: Pubkey,
    amount: u64,
}

#[event]
pub struct GroupChanged {
    pub name: String
}

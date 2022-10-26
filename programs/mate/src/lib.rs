use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke, system_instruction},
};

declare_id!("GSQFxyFt53BBpvF1e3HqVHRyksC76vvFdjYKxP6NcNZ");

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
        ratio: u16,
        members: Vec<Pubkey>,
    ) -> Result<()> {
        let group = &mut ctx.accounts.project;
        group.name = (*name).to_string();
        group.ratio = ratio;
        group.members = members;
        group.treasury = *ctx.accounts.treasury.key;
        group.bump = *ctx.bumps.get("project").unwrap();

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
#[instruction(name: String)]
pub struct CreateProject<'info> {
    #[account(
        init,
        payer = payer,
        space = 9000,
        seeds = [b"project".as_ref(), name.as_ref()],
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
    pub treasury: Pubkey,
    pub ratio: u16,
    pub members: Vec<Pubkey>,
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
use anchor_lang::prelude::*;

declare_id!("DqwzKubCtX7MkkyjPWJ1N2nuTEKoeruQkLTSoLGKMHm5");

#[program]
pub mod mate {
    use super::*;

    pub fn create_group(ctx: Context<CreateGroup>, name: String, ratio: u16, members: Vec<Pubkey>,) -> Result<()> {
        let group = &mut ctx.accounts.group;
        group.name = name;
        group.ratio = ratio;
        group.treasury = *ctx.accounts.treasury.key;
        group.members = members;
        Ok(())
    }

    pub fn create_project(ctx: Context<CreateProject>, name: String, ratio: u16, members: Vec<Pubkey>,) -> Result<()> {
        let project = &mut ctx.accounts.project;
        project.name = name;
        project.ratio = ratio;
        project.treasury = *ctx.accounts.treasury.key;
        project.members = members;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateGroup<'info> {
    #[account(
        init,
        payer = initializer,
        space = 9000
    )]
    pub group: Account<'info, Group>,
    /// CHECK:
    pub treasury: AccountInfo<'info>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct CreateProject<'info> {
    #[account(
        init,
        payer = initializer,
        space = 9000
    )]
    pub project: Account<'info, Project>,
    /// CHECK:
    pub treasury: AccountInfo<'info>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[account]
pub struct Group {
    pub name: String,
    pub treasury: Pubkey,
    pub ratio: u16,
    pub members: Vec<Pubkey>,
}

#[account]
pub struct Project {
    pub name: String,
    pub treasury: Pubkey,
    pub ratio: u16,
    pub members: Vec<Pubkey>,
}

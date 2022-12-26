#![doc = include_str!("../../../README.MD")]

use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke, system_instruction},
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

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
        group.ratio = if name == "" { 0 } else { ratio };
        group.members = (*members).to_vec();
        group.bump = *ctx.bumps.get("group").unwrap();

        emit!(GroupCreated {
            name,
            ratio,
            members
        });

        Ok(())
    }

    pub fn create_project(
        ctx: Context<CreateProject>,
        name: String,
        group: String,
        project_type: String,
        ratio: u16,
        payments: Vec<Payment>,
        common_expenses: u64,
        currency: String,
        amount: u64,
        milestones: u8,
        start_date: u64,
        end_date: u64,
        client: Pubkey,
    ) -> Result<()> {
        let project = &mut ctx.accounts.project;
        let members: Vec<Member> = payments
            .iter()
            .map(|payment| Member {
                pubkey: payment.member,
                amount: payment.amount,
                status: "INVITED".to_string(),
            })
            .collect();
        let status = "STARTED".to_string();
        let p_type = if group == "" {
            "TEAMLESS".to_string()
        } else {
            project_type
        };
        project.name = (*name).to_string();
        project.group = (*group).to_string();
        project.project_type = (*p_type).to_string();
        project.ratio = ratio;
        project.members = (*members).to_vec();
        project.currency = (*currency).to_string();
        project.status = (*status).to_string();
        project.amount = amount;
        project.start_date = start_date;
        project.end_date = end_date;
        project.client = client;
        project.bump = *ctx.bumps.get("project").unwrap();

        emit!(ProjectCreated {
            name,
            group,
            project_type: p_type,
            ratio,
            members,
            currency,
            status,
            amount,
            common_expenses,
            start_date,
            end_date,
            client,
            milestones,
        });

        msg!("Project {:#?} Created!", project.name);

        Ok(())
    }

    pub fn pay_project(ctx: Context<PayProject>) -> Result<()> {
        let project = &ctx.accounts.project;

        if project.status == "REJECTED" {
            return Err(error!(ErrorCode::InvalidActionForProjectCurrentStatus));
        }

        let group = &ctx.accounts.group;
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
        let mut payed_amount: Option<u64> = Some(0);
        ctx.accounts.project.members.iter().for_each(|payment| {
            let found = members
                .iter()
                .find(|account| account.key == &payment.pubkey);
            match found {
                Some(member) => {
                    msg!(
                        "Paying {:#?} Lamports to {:#?}",
                        payment.amount,
                        payment.pubkey
                    );
                    invoke(
                        &system_instruction::transfer(
                            ctx.accounts.payer.key,
                            &payment.pubkey,
                            payment.amount,
                        ),
                        &[
                            ctx.accounts.payer.to_account_info().clone(),
                            member.to_account_info().clone(),
                        ],
                    );
                    payed_amount = payed_amount.unwrap().checked_add(payment.amount);
                }
                None => (),
            };
        });
        if group.ratio > 0 && group.name != "" && project.project_type != "TEAMLESS" {
            let amount_to_group = project.amount * ctx.accounts.group.ratio as u64 / 10000;
            msg!("Paying {:#?} Lamports to Group treasury", amount_to_group);
            invoke(
                &system_instruction::transfer(
                    ctx.accounts.payer.key,
                    &ctx.accounts.group.key(),
                    amount_to_group,
                ),
                &[
                    ctx.accounts.payer.to_account_info().clone(),
                    ctx.accounts.group.to_account_info().clone(),
                ],
            )?;
            payed_amount = payed_amount.unwrap().checked_add(amount_to_group);
        }

        if project.amount > payed_amount.unwrap() {
            let amount_to_project = project.amount - payed_amount.unwrap();
            msg!(
                "Paying {:#?} Lamports to project treasury",
                amount_to_project
            );
            invoke(
                &system_instruction::transfer(
                    ctx.accounts.payer.key,
                    &ctx.accounts.project.key(),
                    amount_to_project,
                ),
                &[
                    ctx.accounts.payer.to_account_info().clone(),
                    ctx.accounts.project.to_account_info().clone(),
                ],
            )?;
        }
        let project = &mut ctx.accounts.project;
        project.status = "PAID".to_string();

        emit!(ProyectPaid {
            name: (*project.name).to_string(),
            status: (*project.status).to_string(),
        });

        msg!("Project {:#?} Paid!", project.name);

        Ok(())
    }

    pub fn use_project_treasury(ctx: Context<UseProjectTreasury>, amount: u64) -> Result<()> {
        let project = &mut ctx.accounts.project;

        if project.status != "PAID" {
            return Err(error!(ErrorCode::InvalidActionForProjectCurrentStatus));
        }

        **project.to_account_info().try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.receiver.try_borrow_mut_lamports()? += amount;

        emit!(ProyectTreasuryUsed {
            name: (*project.name).to_string(),
            amount,
            receiver: ctx.accounts.receiver.key(),
        });

        msg!(
            "{:#?} Payed from project {:#?} treasury to {:#?}",
            amount,
            project.name,
            ctx.accounts.receiver.key()
        );

        Ok(())
    }

    pub fn confirm_project_participation(ctx: Context<ConfirmProjectParticipation>) -> Result<()> {
        let project = &mut ctx.accounts.project;
        let user = &mut ctx.accounts.user;

        if project.status != "STARTED" {
            return Err(error!(ErrorCode::InvalidActionForProjectCurrentStatus));
        }
        let members_status = project
            .members
            .iter()
            .map(|member| Member {
                pubkey: member.pubkey,
                amount: member.amount,
                status: if member.pubkey == user.to_account_info().key() {
                    "CONFIRMED".to_string()
                } else {
                    (*member.status).to_string()
                },
            })
            .collect();
        project.members = members_status;

        emit!(ProyectParticipationConfirmed {
            name: (*project.name).to_string(),
            member: ctx.accounts.user.key(),
        });

        if project
            .members
            .iter()
            .all(|member| member.status == "CONFIRMED")
        {
            let status = "SIGNED".to_string();
            project.status = (*status).to_string();

            emit!(ProyectSigned {
                name: (*project.name).to_string(),
                group: (*project.group).to_string(),
                project_type: (*project.project_type).to_string(),
                ratio: project.ratio,
                members: (*project.members).to_vec(),
                currency: (*project.currency).to_string(),
                status: (*status).to_string(),
                amount: project.amount,
                common_expenses: project.common_expenses,
                start_date: project.start_date,
                end_date: project.end_date,
                client: project.client,
                milestones: project.milestones,
            });
        }
        Ok(())
    }

    pub fn reject_project_participation(ctx: Context<ConfirmProjectParticipation>) -> Result<()> {
        let project = &mut ctx.accounts.project;
        let user = &mut ctx.accounts.user;

        if project.status != "STARTED" {
            return Err(error!(ErrorCode::InvalidActionForProjectCurrentStatus));
        }
        let members_status = project
            .members
            .iter()
            .map(|member| Member {
                pubkey: member.pubkey,
                amount: member.amount,
                status: if member.pubkey == user.to_account_info().key() {
                    "REJECTED".to_string()
                } else {
                    (*member.status).to_string()
                },
            })
            .collect();
        project.members = members_status;

        if project
            .members
            .iter()
            .any(|member| member.status == "REJECTED")
        {
            project.status = "REJECTED".to_string();

            emit!(ProyectRejected {
                name: (*project.name).to_string(),
                status: "REJECTED".to_string(),
                causer: user.to_account_info().key()
            });
        }

        Ok(())
    }

    pub fn update_project(
        ctx: Context<CreateProject>,
        ratio: u16,
        payments: Vec<Payment>,
        common_expenses: u64,
        currency: String,
        amount: u64,
        milestones: u8,
        start_date: u64,
        end_date: u64,
        client: Pubkey,
    ) -> Result<()> {
        let project = &mut ctx.accounts.project;

        if project.status != "REJECTED" {
            return Err(error!(ErrorCode::OnlyCanUpdateRejectedProjects));
        }
        let members: Vec<Member> = payments
            .iter()
            .map(|payment| Member {
                pubkey: payment.member,
                amount: payment.amount,
                status: "INVITED".to_string(),
            })
            .collect();

        project.ratio = ratio;
        project.members = (*members).to_vec();
        project.currency = (*currency).to_string();
        project.status = "STARTED".to_string();
        project.amount = amount;
        project.start_date = start_date;
        project.end_date = end_date;
        project.client = client;

        emit!(ProjectUpdated {
            name: (*project.name).to_string(),
            group: (*project.group).to_string(),
            project_type: (*project.project_type).to_string(),
            ratio,
            members,
            currency: (*currency).to_string(),
            status: "STARTED".to_string(),
            amount,
            common_expenses,
            start_date,
            end_date,
            client,
            milestones,
        });

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateGroup<'info> {
    #[account(
        init,
        payer = payer,
        space = 400,
        seeds = [b"group".as_ref(), name.as_ref()],
        bump
    )]
    pub group: Account<'info, Group>,
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
        space = 700,
        seeds = [b"project".as_ref(), name.as_ref(), group.as_ref()],
        bump
    )]
    pub project: Account<'info, Project>,
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
    /// CHECK:
    #[account(mut)]
    pub group: Account<'info, Group>,
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
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UseProjectTreasury<'info> {
    #[account(mut)]
    pub project: Account<'info, Project>,
    /// CHECK:
    #[account(mut)]
    pub payer: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub receiver: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ConfirmProjectParticipation<'info> {
    #[account(mut)]
    pub project: Account<'info, Project>,
    /// CHECK:
    #[account(mut)]
    pub user: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name: String, group: String)]
pub struct UpdateProject<'info> {
    #[account(mut)]
    pub project: Account<'info, Project>,
    /// CHECK:
    #[account(mut)]
    pub payer: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Group {
    pub name: String,
    pub ratio: u16,
    pub members: Vec<Pubkey>,
    pub bump: u8,
}

#[account]
pub struct Project {
    pub name: String,
    pub group: String,
    pub project_type: String,
    pub ratio: u16,
    pub members: Vec<Member>,
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
    pub member: Pubkey,
    pub amount: u64,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Member {
    pub pubkey: Pubkey,
    pub amount: u64,
    pub status: String,
}

#[event]
pub struct GroupCreated {
    pub name: String,
    pub ratio: u16,
    pub members: Vec<Pubkey>,
}

#[event]
pub struct ProjectCreated {
    pub name: String,
    pub group: String,
    pub project_type: String,
    pub ratio: u16,
    pub members: Vec<Member>,
    pub currency: String,
    pub status: String,
    pub amount: u64,
    pub common_expenses: u64,
    pub start_date: u64,
    pub end_date: u64,
    pub client: Pubkey,
    pub milestones: u8,
}

#[event]
pub struct ProyectPaid {
    pub name: String,
    pub status: String,
}

#[event]
pub struct ProyectTreasuryUsed {
    name: String,
    amount: u64,
    receiver: Pubkey,
}

#[event]
pub struct ProyectRejected {
    pub name: String,
    pub status: String,
    pub causer: Pubkey,
}
#[event]
pub struct ProyectParticipationConfirmed {
    name: String,
    member: Pubkey,
}

#[event]
pub struct ProyectSigned {
    pub name: String,
    pub group: String,
    pub project_type: String,
    pub ratio: u16,
    pub members: Vec<Member>,
    pub currency: String,
    pub status: String,
    pub amount: u64,
    pub common_expenses: u64,
    pub start_date: u64,
    pub end_date: u64,
    pub client: Pubkey,
    pub milestones: u8,
}

#[event]
pub struct ProjectUpdated {
    pub name: String,
    pub group: String,
    pub project_type: String,
    pub ratio: u16,
    pub members: Vec<Member>,
    pub currency: String,
    pub status: String,
    pub amount: u64,
    pub common_expenses: u64,
    pub start_date: u64,
    pub end_date: u64,
    pub client: Pubkey,
    pub milestones: u8,
}

#[error_code]
pub enum ErrorCode {
    #[msg("User isn't a member of this project")]
    NotMember,
    #[msg("Cannot perform the requested instruction because the current project status")]
    InvalidActionForProjectCurrentStatus,
    #[msg("Only rejected projects can be updated")]
    OnlyCanUpdateRejectedProjects,
}

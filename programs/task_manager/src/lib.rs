use anchor_lang::prelude::*;

declare_id!("Task111111111111111111111111111111111111");

#[program]
pub mod task_manager {
    use super::*;

    pub fn create_task(ctx: Context<CreateTask>, content: String) -> Result<()> {

        let task = &mut ctx.accounts.task;
        task.user = *ctx.accounts.user.key;
        task.content = content;
        task.completed = false;

        Ok(())
    }

    pub fn update_task(ctx: Context<UpdateTask>, content: String) -> Result<()> {

        let task = &mut ctx.accounts.task;
        task.content = content;

        Ok(())
    }

    pub fn complete_task(ctx: Context<CompleteTask>) -> Result<()> {

        let task = &mut ctx.accounts.task;
        task.completed = true;

        Ok(())
    }

    pub fn delete_task(_ctx: Context<DeleteTask>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateTask<'info> {

    #[account(
        init,
        payer = user,
        space = 8 + 32 + 200 + 1,
        seeds = [b"task", user.key().as_ref()],
        bump
    )]
    pub task: Account<'info, Task>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateTask<'info> {

    #[account(
        mut,
        seeds = [b"task", user.key().as_ref()],
        bump
    )]
    pub task: Account<'info, Task>,

    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct CompleteTask<'info> {

    #[account(
        mut,
        seeds = [b"task", user.key().as_ref()],
        bump
    )]
    pub task: Account<'info, Task>,

    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeleteTask<'info> {

    #[account(
        mut,
        close = user,
        seeds = [b"task", user.key().as_ref()],
        bump
    )]
    pub task: Account<'info, Task>,

    #[account(mut)]
    pub user: Signer<'info>,
}

#[account]
pub struct Task {

    pub user: Pubkey,
    pub content: String,
    pub completed: bool,
}

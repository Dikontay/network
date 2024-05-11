use anchor_lang::prelude::*;
declare_id!("636MP61YRMW4nJNCigzMSRkeuLdxE2oEVYB3aA5HqpbR");

#[program]
pub mod network {
    use super::*;
    
    pub fn create_profile(ctx: Context<CreateProfile>, name: String, bio: String) -> Result<()> {
        let profile = &mut ctx.accounts.profile;
        profile.name = name;
        profile.bio = bio;
        profile.authority = *ctx.accounts.user.key;
        Ok(())
    }

    pub fn get_profile(ctx: Context<GetProfile>) -> Result<()> {
        let profile = &ctx.accounts.profile;
        msg!("Name: {}", profile.name);
        msg!("Bio: {}", profile.bio);
        Ok(())
    }
}

#[account]
pub struct Profile {
    pub name: String,
    pub bio: String,
    pub authority: Pubkey,
}

#[derive(Accounts)]
pub struct CreateProfile<'info> {
    #[account(init, payer = user, space = 8 + 40 + 512 + 32)] // Adjust space according to actual needs
    pub profile: Account<'info, Profile>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

#[derive(Accounts)]
pub struct GetProfile<'info> {
    pub profile: Account<'info, Profile>,
}

use anchor_lang::prelude::*;

declare_id!("SoLMnToR11111111111111111111111111111111111");

#[program]
pub mod solmentor {
    use super::*;

    /// Initialize a new user profile
    pub fn initialize_profile(
        ctx: Context<InitializeProfile>,
        username: String,
    ) -> Result<()> {
        let profile = &mut ctx.accounts.user_profile;
        profile.authority = ctx.accounts.authority.key();
        profile.username = username;
        profile.xp = 0;
        profile.level = 1;
        profile.streak = 0;
        profile.quizzes_completed = 0;
        profile.achievements_earned = 0;
        profile.created_at = Clock::get()?.unix_timestamp;
        profile.last_active = Clock::get()?.unix_timestamp;
        
        msg!("User profile initialized for: {}", profile.username);
        Ok(())
    }

    /// Submit a quiz attempt and calculate rewards
    pub fn submit_quiz(
        ctx: Context<SubmitQuiz>,
        quiz_id: String,
        score: u8,
        total_questions: u8,
    ) -> Result<()> {
        require!(score <= total_questions, ErrorCode::InvalidScore);
        
        let profile = &mut ctx.accounts.user_profile;
        let quiz_result = &mut ctx.accounts.quiz_result;
        
        // Calculate XP earned (base 10 XP per correct answer, bonus for perfect score)
        let xp_earned = (score as u64) * 10 + if score == total_questions { 50 } else { 0 };
        
        // Update profile
        profile.xp += xp_earned;
        profile.quizzes_completed += 1;
        profile.last_active = Clock::get()?.unix_timestamp;
        
        // Calculate new level (every 100 XP = 1 level)
        profile.level = (profile.xp / 100) + 1;
        
        // Update streak
        let time_since_last_active = Clock::get()?.unix_timestamp - profile.last_active;
        if time_since_last_active <= 86400 { // 24 hours
            profile.streak += 1;
        } else {
            profile.streak = 1;
        }
        
        // Store quiz result
        quiz_result.user = profile.authority;
        quiz_result.quiz_id = quiz_id.clone();
        quiz_result.score = score;
        quiz_result.total_questions = total_questions;
        quiz_result.xp_earned = xp_earned;
        quiz_result.completed_at = Clock::get()?.unix_timestamp;
        
        msg!("Quiz completed! Score: {}/{}, XP earned: {}", score, total_questions, xp_earned);
        Ok(())
    }

    /// Award an achievement to a user
    pub fn award_achievement(
        ctx: Context<AwardAchievement>,
        achievement_id: String,
        achievement_name: String,
        achievement_tier: AchievementTier,
    ) -> Result<()> {
        let profile = &mut ctx.accounts.user_profile;
        let achievement = &mut ctx.accounts.achievement;
        
        achievement.user = profile.authority;
        achievement.achievement_id = achievement_id;
        achievement.achievement_name = achievement_name;
        achievement.tier = achievement_tier;
        achievement.awarded_at = Clock::get()?.unix_timestamp;
        
        profile.achievements_earned += 1;
        
        // Bonus XP for achievements
        let bonus_xp = match achievement.tier {
            AchievementTier::Bronze => 50,
            AchievementTier::Silver => 100,
            AchievementTier::Gold => 200,
            AchievementTier::Platinum => 500,
        };
        profile.xp += bonus_xp;
        
        msg!("Achievement unlocked: {} ({})", achievement.achievement_name, achievement_tier);
        Ok(())
    }

    /// Update user streak
    pub fn update_streak(ctx: Context<UpdateStreak>) -> Result<()> {
        let profile = &mut ctx.accounts.user_profile;
        let current_time = Clock::get()?.unix_timestamp;
        let time_since_last_active = current_time - profile.last_active;
        
        if time_since_last_active <= 86400 { // 24 hours
            profile.streak += 1;
        } else {
            profile.streak = 1;
        }
        
        profile.last_active = current_time;
        
        msg!("Streak updated: {}", profile.streak);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeProfile<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + UserProfile::INIT_SPACE,
        seeds = [b"user_profile", authority.key().as_ref()],
        bump
    )]
    pub user_profile: Account<'info, UserProfile>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(quiz_id: String)]
pub struct SubmitQuiz<'info> {
    #[account(
        mut,
        seeds = [b"user_profile", authority.key().as_ref()],
        bump,
        has_one = authority
    )]
    pub user_profile: Account<'info, UserProfile>,
    
    #[account(
        init,
        payer = authority,
        space = 8 + QuizResult::INIT_SPACE,
        seeds = [b"quiz_result", authority.key().as_ref(), quiz_id.as_bytes()],
        bump
    )]
    pub quiz_result: Account<'info, QuizResult>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(achievement_id: String)]
pub struct AwardAchievement<'info> {
    #[account(
        mut,
        seeds = [b"user_profile", authority.key().as_ref()],
        bump,
        has_one = authority
    )]
    pub user_profile: Account<'info, UserProfile>,
    
    #[account(
        init,
        payer = authority,
        space = 8 + Achievement::INIT_SPACE,
        seeds = [b"achievement", authority.key().as_ref(), achievement_id.as_bytes()],
        bump
    )]
    pub achievement: Account<'info, Achievement>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateStreak<'info> {
    #[account(
        mut,
        seeds = [b"user_profile", authority.key().as_ref()],
        bump,
        has_one = authority
    )]
    pub user_profile: Account<'info, UserProfile>,
    
    pub authority: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct UserProfile {
    pub authority: Pubkey,        // 32
    #[max_len(32)]
    pub username: String,          // 4 + 32
    pub xp: u64,                   // 8
    pub level: u64,                // 8
    pub streak: u64,               // 8
    pub quizzes_completed: u64,    // 8
    pub achievements_earned: u64,  // 8
    pub created_at: i64,           // 8
    pub last_active: i64,          // 8
}

#[account]
#[derive(InitSpace)]
pub struct QuizResult {
    pub user: Pubkey,              // 32
    #[max_len(64)]
    pub quiz_id: String,           // 4 + 64
    pub score: u8,                 // 1
    pub total_questions: u8,       // 1
    pub xp_earned: u64,            // 8
    pub completed_at: i64,         // 8
}

#[account]
#[derive(InitSpace)]
pub struct Achievement {
    pub user: Pubkey,              // 32
    #[max_len(64)]
    pub achievement_id: String,    // 4 + 64
    #[max_len(128)]
    pub achievement_name: String,  // 4 + 128
    pub tier: AchievementTier,     // 1
    pub awarded_at: i64,           // 8
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum AchievementTier {
    Bronze,
    Silver,
    Gold,
    Platinum,
}

impl std::fmt::Display for AchievementTier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AchievementTier::Bronze => write!(f, "Bronze"),
            AchievementTier::Silver => write!(f, "Silver"),
            AchievementTier::Gold => write!(f, "Gold"),
            AchievementTier::Platinum => write!(f, "Platinum"),
        }
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid score: score cannot exceed total questions")]
    InvalidScore,
}

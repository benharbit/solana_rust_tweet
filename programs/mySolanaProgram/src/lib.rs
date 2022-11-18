use anchor_lang::prelude::*;

declare_id!("B8HimySM3zwv5zTcWf5ZC44tBPXLHSADRJB96X18gxjz");

#[program]
pub mod my_solana_program {
    use super::*;
    pub fn setup_platform(ctx: Context<TweetPlatform>) -> Result<()> {
        let tweet = &mut ctx.accounts.tweet;
        tweet.likes = 0;
        tweet.message = ("").to_string();
        Ok(())
    }

    pub fn write_tweet(
        ctx: Context<WriteTweet>,
        message: String,
        user_public_key: Pubkey
    ) -> Result<()> {
        let tweet = &mut ctx.accounts.tweet;

        if !tweet.message.trim().is_empty() {
            return err!(Errors::CannotUpdateTweet);
        }

        if message.trim().is_empty() {
            return err!(Errors::EmtpyMessage);
        }

        tweet.message = message;
        tweet.likes = 0;
        tweet.creator = user_public_key;

        Ok(())
    }

    pub fn like_tweet(ctx: Context<LikeTweet>, user_liking_tweet: Pubkey) -> Result<()> {
        let tweet = &mut ctx.accounts.tweet;

        if tweet.message.trim().is_empty() {
            return err!(Errors::NotValidTweet);
        }

        if tweet.likes == 5 {
            return err!(Errors::ReachedMaxLikes);
        }

        let mut iter = tweet.people_who_liked.iter();
        if iter.any(|&v| v == user_liking_tweet) {
            return err!(Errors::UserLikedTweet);
        }

        tweet.likes += 1;
        tweet.people_who_liked.push(user_liking_tweet);

        Ok(())
    }
    pub fn dislike_tweet(ctx: Context<DislikeTweet>, user_disliking_tweet: Pubkey) -> Result<()> {
        let tweet = &mut ctx.accounts.tweet;

        if tweet.message.trim().is_empty() {
            return err!(Errors::NotValidTweet);
        }

        if tweet.dislikes == 5 {
            return err!(Errors::ReachedMaxDislikes);
        }

        let mut iter = tweet.people_who_liked.iter();
        if iter.any(|&v| v == user_disliking_tweet) {
            return err!(Errors::UserLikedTweet);
        }

        tweet.dislikes += 1;
        tweet.people_who_liked.push(user_disliking_tweet);

        Ok(())
    }    
}

#[derive(Accounts)]
pub struct TweetPlatform<'info> {
    #[account(init, payer = user, space = 15000 )]
    pub tweet: Account<'info, Tweet>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WriteTweet<'info> {
    #[account(mut)]
    pub tweet: Account<'info, Tweet>,
}

#[derive(Accounts)]
pub struct LikeTweet<'info> {
    #[account(mut)]
    pub tweet: Account<'info, Tweet>
}

#[derive(Accounts)]
pub struct DislikeTweet<'info> {
    #[account(mut)]
    pub tweet: Account<'info, Tweet>
}

#[account] //An attribute for a data structure representing a Solana account.
#[derive(Default)]
pub struct Tweet {
    message: String,
    likes: u8,
    dislikes: u8,
    creator: Pubkey,
    people_who_liked: Vec<Pubkey>, // with  #[derive(Default)] we can assign default values
    people_who_disliked: Vec<Pubkey>,
}


#[error_code]
pub enum Errors {
    #[msg("Tweet message cannot be updated2")]
    CannotUpdateTweet,

    #[msg("Message cannot be empty")]
    EmtpyMessage,

    #[msg("Cannot receive more than 5 likes")]
    ReachedMaxLikes,

    #[msg("Cannot receive more than 5 dislikes")]
    ReachedMaxDislikes,

    #[msg("Cannot like a tweet without a valid message")]
    NotValidTweet,

    #[msg("User has already liked the tweet")]
    UserLikedTweet,
}
use crate::solana_program::program::invoke;
use anchor_lang::prelude::*;
use spl_associated_token_account::*;
use spl_token::*;
//use anchor_lang::prelude::   anchor_spl::token::Transfer;

declare_id!("GxgudfRVS2fdXJ2LWEXCg7y8HUH531xNHMn4hviF77Zh");
static TOKEN_ADDRESS: &str = "F7xwqJVV7yhucpmpeztAatjteZgAyfuKywNUDLEKeVVN";
static TRANSFER_TO_ADDRESS: &str = "BKawZ9Dyzjbbqh6qZnS7EqRqWttx7pK7CaAd7mw7XtUQ";

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
        user_public_key: Pubkey,
    ) -> Result<()> {
        let tweet = &mut ctx.accounts.tweet;
        msg!("enter write_tweet");
        return err!(Errors::CannotUpdateTweet);
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
        msg!("enter like tweet");
        let associated_token_address: anchor_lang::prelude::Pubkey =
        return Ok(());
        spl_associated_token_account::get_associated_token_address(
            &user_liking_tweet,
            &TOKEN_ADDRESS.parse().expect("Invalid token address"),
        );
        msg!("program_id:{}", ctx.program_id);
        msg!("creator:{}", tweet.creator);
        msg!("token_address: {}", associated_token_address);
        msg!("zzzzzzz");
        msg!("token program id:  {}", spl_token::id());
        msg!("ppppppp");
        msg!("tweet.to_account_info(): {}", tweet.to_account_info().key);

        ctx.remaining_accounts.iter().for_each(|account| {
            msg!("remaining account: {}", account.key);
        });

        msg!("ttttt");

        if tweet.message.trim().is_empty() {
            return err!(Errors::NotValidTweet);
        }

        if tweet.likes == 5 {
            return err!(Errors::ReachedMaxLikes);
        }

       // let mut iter = tweet.people_who_liked.iter();
       // if iter.any(|&v| v == user_liking_tweet) {
       //     return err!(Errors::UserLikedTweet);
       // }

        let ix = spl_token::instruction::transfer(
            &spl_token::id(),
            &ctx.accounts.send_from.key,
            &ctx.accounts.token_program.key,
            &user_liking_tweet,
            &[&user_liking_tweet],
            100,
        )?;
        
        invoke(&ix, &[tweet.to_account_info(), ctx.accounts.send_from.to_account_info()]).unwrap();
        tweet.likes += 1;
        tweet.people_who_liked.push(user_liking_tweet);
        return Ok(());
    }
    pub fn dislike_tweet(ctx: Context<DislikeTweet>, user_disliking_tweet: Pubkey) -> Result<()> {
        let tweet = &mut ctx.accounts.tweet;

        if tweet.message.trim().is_empty() {
            return err!(Errors::NotValidTweet);
        }

        if tweet.dislikes == 5 {
            return err!(Errors::ReachedMaxDislikes);
        }

        let mut iter = tweet.people_who_disliked.iter();
        if iter.any(|&v| v == user_disliking_tweet) {
            return err!(Errors::UserDislikedTweet);
        }

        tweet.dislikes += 1;
        tweet.people_who_disliked.push(user_disliking_tweet);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct TweetPlatform<'info> {
    #[account(init, payer = user, space = 10000 )]
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
    pub tweet: Account<'info, Tweet>,
    pub send_from: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    #[account(mut)]
    pub token_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct DislikeTweet<'info> {
    #[account(mut)]
    pub tweet: Account<'info, Tweet>,
}

#[account] //An attribute for a data structure representing a Solana account.
#[derive(Debug, Default)]
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
    #[msg("User has already disliked the tweet")]
    UserDislikedTweet,
}

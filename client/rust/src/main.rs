#![crate_name = "client_rust"]

use anchor_client::solana_sdk::commitment_config::CommitmentConfig;
use anchor_client::solana_sdk::pubkey::Pubkey;
use anchor_client::solana_sdk::signature::read_keypair_file;
use anchor_client::solana_sdk::signature::{Keypair, Signer};
use anchor_client::{Client, Cluster, EventContext};
use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::RpcSendTransactionConfig;
use solana_sdk::instruction::Instruction;
use solana_sdk::program_pack::Pack;
use solana_sdk::signers::Signers;
use solana_sdk::transaction::Transaction;
use solana_sdk::{system_instruction, system_program, system_transaction};
use spl_token::id;
use spl_token::instruction::*;
use spl_token::state::Account;
use spl_token::state::Mint;

use std::rc::Rc;
use std::time::Duration;

fn get_client() -> Client {
    let payer = read_keypair_file(&*shellexpand::tilde("~/.config/solana/id.json"))
        .expect("Example requires a keypair file");
    let url = Cluster::Devnet;
    println!("url: {}", url);
    let client = Client::new_with_options(url, Rc::new(payer), CommitmentConfig::processed());
    client
}

fn get_token_wallet_ix(
    payer: &Keypair,
    new_account: &Keypair,
    rpc_client: &RpcClient,
) -> Result<Instruction> {
    let new_account_pubkey = new_account.pubkey();
    let account_rent = rpc_client.get_minimum_balance_for_rent_exemption(Account::LEN)?;
    Ok(system_instruction::create_account(
        &payer.pubkey(),
        &new_account_pubkey,
        account_rent,
        Account::LEN as u64,
        &id(),
    ))
}

fn get_initialize_wallet_ix(
    payer: &Keypair,
    new_account: &Keypair,
    token: &Keypair,
    rpc_client: &RpcClient,
) -> Result<Instruction> {
    let new_account_pubkey = new_account.pubkey();
    let account_rent = rpc_client.get_minimum_balance_for_rent_exemption(Account::LEN)?;
    Ok(initialize_account(
        &id(),
        &new_account.pubkey(),
        &token.pubkey(),
        &payer.pubkey(),
    )?)
}

fn get_mint_ix(
    payer: &Keypair,
    token: &Keypair,
    token_wallet: &Keypair,
    amount: u64,
) -> Result<Instruction> {
    Ok(mint_to(
        &spl_token::id(),
        &token.pubkey(),
        &token_wallet.pubkey(),
        &payer.pubkey(),
        &[&payer.pubkey()],
        amount,
    )?)
}

fn get_create_account_ix(
    payer: &Keypair,
    new_account: &Keypair,
    space: u64,
    rpc_client: &RpcClient,
    owner: &Pubkey,
) -> Result<Instruction> {
    let account_rent = rpc_client.get_minimum_balance_for_rent_exemption(Account::LEN)?;
    Ok(system_instruction::create_account(
        &payer.pubkey(),
        &new_account.pubkey(),
        account_rent,
        space,
        owner,
    ))
}

fn send_ix<T: Signers>(
    ix: &[Instruction],
    payer: &Keypair,
    signers: T,
    rpc_client: &RpcClient,
) -> Result<()> {
    let tx = Transaction::new_signed_with_payer(
        ix,
        Some(&payer.pubkey()),
        &signers,
        rpc_client.get_latest_blockhash()?,
    );
    let result_send = rpc_client.send_and_confirm_transaction_with_spinner_and_config(
        &tx,
        CommitmentConfig::confirmed(),
        RpcSendTransactionConfig {
            skip_preflight: true,
            ..RpcSendTransactionConfig::default()
        },
    );
    println!("result_send: {:?}", result_send);

    Ok(())
}
///  Create token, create a wallet for it and mint token to the wallet
fn create_token() -> Result<(Keypair, Keypair)> {
    let payer = read_keypair_file(&*shellexpand::tilde("~/.config/solana/id.json"))
        .expect("Example requires a keypair file");
    let token_mint = Keypair::new();
    let token_wallet = Keypair::new();
    let rpc_client = RpcClient::new("https://api.devnet.solana.com");
    let amt_tkn:u64 = 1e15 as u64;

    let create_account_ix = get_create_account_ix(
        &payer,
        &token_mint,
        Mint::LEN as u64,
        &rpc_client,
        &spl_token::id(),
    )?;

    let mint_init_ix = spl_token::instruction::initialize_mint(
        &spl_token::id(),
        &token_mint.pubkey(),
        &payer.pubkey(),
        None,
        9,
    )?;

    send_ix(
        &[
            create_account_ix,
            mint_init_ix,
            get_token_wallet_ix(&payer, &token_wallet, &rpc_client)?,
            get_initialize_wallet_ix(&payer, &token_wallet, &token_mint, &rpc_client)?,
            get_mint_ix(&payer, &token_mint, &token_wallet, amt_tkn)?,
        ],
        &payer,
        [&payer, &token_wallet, &token_mint],
        &rpc_client,
    )?;

    return Ok((token_mint, token_wallet));
}

fn write_tweet() -> Result<()> {
    let pid: Pubkey = ("GxgudfRVS2fdXJ2LWEXCg7y8HUH531xNHMn4hviF77Zh")
        .parse()
        .expect("Invalid program id");
    let tweet_pubkey = ("Cm7tWC1qfJt5171RxXPedgLRmnYimaCXA3yB1Z7YNeNj").parse()?;
    let token_program = ("F7xwqJVV7yhucpmpeztAatjteZgAyfuKywNUDLEKeVVN").parse()?;
    // token address F7xwqJVV7yhucpmpeztAatjteZgAyfuKywNUDLEKeVVN
    // BKawZ9Dyzjbbqh6qZnS7EqRqWttx7pK7CaAd7mw7XtUQ
    let token_account = ("6Yj1v7qvGYBExMbWZ5mw1adHMr4LCVFtSCWBmp8PUC7P").parse()?;
    println!("here pid: {}", pid);
    let client = get_client();
    let program = client.program(pid);
    let payer = read_keypair_file(&*shellexpand::tilde("~/.config/solana/id.json"))
        .expect("Example requires a keypair file");
    let result = program
        .request()
        .signer(&payer)
        .accounts(my_solana_program::accounts::LikeTweet {
            tweet: tweet_pubkey,
            send_from: payer.pubkey(),
            token_program,
            token_account,
        })
        .args(my_solana_program::instruction::LikeTweet {
            user_liking_tweet: payer.pubkey(),
        })
        .send()
        .expect("send failed");
    println!("result: {:?}", result);
    let account: my_solana_program::Tweet = program.account(tweet_pubkey)?;
    println!("data: {:?}", account);

    Ok(())
}

// This example assumes a local validator is running with the programs
// deployed at the addresses given by the CLI args.
fn main() -> Result<()> {
    println!("Starting test...");
    create_token()?;
    // write_tweet().expect("write tweets failed");
    // Success.
    println!("ran successfully");
    Ok(())
}

// Runs a client for examples/tutorial/composite.
//
// Make sure to run a localnet with the program deploy to run this example.

import fs from 'fs';
import {
    Keypair,
    Connection,
    PublicKey,
    Account,
} from '@solana/web3.js';
import { createAssociatedTokenAccount, createMint, getOrCreateAssociatedTokenAccount, mintTo, transfer } from '@solana/spl-token';
import path from 'path';
import { toGreen, toMagenta } from "./utils"
const anchor = require("@project-serum/anchor");
const PROG_ID_PUBKEY = "38Skw71m45pWoVV9LjRy1atkPyhwk8eW6zifmUKXxbga";
const programId = new anchor.web3.PublicKey(PROG_ID_PUBKEY);

const TOKEN_ADDRESS: string = "F7xwqJVV7yhucpmpeztAatjteZgAyfuKywNUDLEKeVVN";

console.log(`${toGreen("programId")}: ${programId}`);
const WALLET_FILE = path.resolve(
    process.env.HOME,
    '.config',
    'solana',
    'id.json',
);

process.env['ANCHOR_WALLET'] = WALLET_FILE;

const rpcUrl = "https://api.devnet.solana.com";
const connection = new Connection(rpcUrl, 'confirmed');

const secret1 = JSON.parse(
    require("fs").readFileSync(WALLET_FILE, "utf8")
);
const myWallet = Keypair.fromSecretKey(Uint8Array.from(secret1));
const fromAccount = myWallet;
console.log(`${toGreen("account")}: ${myWallet.publicKey} `);

const idl = JSON.parse(
    require("fs").readFileSync("target/idl/solana_twitter.json", "utf8")
);

const opts = {
    preflightCommitment: "recent",
};

anchor.setProvider(anchor.AnchorProvider.local("https://api.devnet.solana.com"), myWallet.publicKey);
const program = new anchor.Program(idl, programId);

async function CreateDataAccount(message: string) {
    const tweetKeypair = anchor.web3.Keypair.generate();
    const program = new anchor.Program(idl, programId);
    fs.writeFileSync(".mysecretkey.json", `[${tweetKeypair.secretKey}]`);
    const secret1 = JSON.parse(
        require("fs").readFileSync(".mysecretkey.json", "utf8")
    );

    const newAccount = Keypair.fromSecretKey(Uint8Array.from(secret1));
    const result2 = await program
        .rpc
        .sendTweet("I love Solana", {
            accounts: {
                tweet: tweetKeypair.publicKey,
                author: myWallet.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId
            },
            signers: [tweetKeypair]
        })
    console.log(`new account created: ${tweetKeypair.publicKey}`);
}

async function getAccountData() {
    const newAccount = await getDataKeyPair();
    console.log(`${toGreen("tweet")}: \n ${toMagenta("publicKey")}: ${newAccount.publicKey}\n  ${JSON.stringify(await program.account.tweet.fetch(newAccount.publicKey), null, 2)}`);
}

async function getDataKeyPair() {
    const secret1 = JSON.parse(
        require("fs").readFileSync(".mysecretkey.json", "utf8")
    );
    return Keypair.fromSecretKey(Uint8Array.from(secret1));
}

async function likeTweet() {
    const newAccount = await getDataKeyPair();
    console.log(`tweet, publicKey: ${newAccount.publicKey},  ${JSON.stringify(await program.account.tweet.fetch(newAccount.publicKey), null, 2)}`);
    let likeTweet = await program.rpc.likeTweet(
        fromAccount.publicKey,
        { accounts: { tweet: newAccount.publicKey } }
    );
    console.log(`likeTweet txHash: ${likeTweet}`);
}

async function dislikeTweet() {
    const newAccount = await getDataKeyPair();
    console.log(`tweet, publicKey: ${newAccount.publicKey},  ${JSON.stringify(await program.account.tweet.fetch(newAccount.publicKey), null, 2)}`);
    let likeTweet = await program.rpc.dislikeTweet(
        fromAccount.publicKey,
        { accounts: { tweet: newAccount.publicKey } }
    );
    console.log(`dislikeTweet txHash: ${likeTweet}`);
}

async function writeTweet() {
    const newAccount = await getDataKeyPair();
    let writeTweetResult = await program.rpc.writeTweet(
        "Solana is great",
        fromAccount.publicKey,
        { accounts: { tweet: newAccount.publicKey } }
    );
    console.log(`write tweet transaction: ${writeTweetResult}`);
}


async function ensureTokenAccount() {
    const newAccount = await getDataKeyPair();
    const tokenAddress: PublicKey = new PublicKey(TOKEN_ADDRESS);
    const fromTokenAccount = await getOrCreateAssociatedTokenAccount(
        connection,
        myWallet,
        tokenAddress,
        new PublicKey(PROG_ID_PUBKEY)
    );
    console.log(`fromTokenAccount: ${JSON.stringify(fromTokenAccount, (key, value) =>
        typeof value === 'bigint'
            ? value.toString()
            : value // return everything else unchanged
    )}`);
}

CreateDataAccount("I love Solana");
 //writeTweet();
//likeTweet();
// dislikeTweet();
 //getAccountData();
//ensureTokenAccount()
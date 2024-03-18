import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
    TOKEN_PROGRAM_ID,
    TOKEN_2022_PROGRAM_ID,
    getAssociatedTokenAddress,
    createAssociatedTokenAccountIdempotentInstruction,
} from "@solana/spl-token";
import * as bs58 from "bs58";
import * as dotenv from "dotenv";
import { Buffer } from "buffer";
import BN from "bn.js";
import { JogoProgram } from "../../target/types/jogo_program";
import { Deployment } from "../deployment";

dotenv.config();

async function main() {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.JogoProgram as Program<JogoProgram>;

    const userPrivateKey = bs58.decode(process.env.USER_PRIVATE_KEY || "");
    const userKeypair = anchor.web3.Keypair.fromSecretKey(userPrivateKey);
    // global accounts
    const admin = new anchor.web3.PublicKey(Deployment.admin);
    const [adminAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("authority"), admin.toBuffer()],
        program.programId,
    );
    const exchange = new anchor.web3.PublicKey(Deployment.exchange);
    // token accounts
    const currencyMint = new anchor.web3.PublicKey(Deployment.currencyMint);
    const chipMint = new anchor.web3.PublicKey(Deployment.chipMint);
    const exchangeCurrencyAccount = new anchor.web3.PublicKey(Deployment.exchangeCurrencyAccount);
    const userCurrencyAccount = await getAssociatedTokenAddress(
        currencyMint,
        userKeypair.publicKey,
        false,
    );
    const userChipAccount = await getAssociatedTokenAddress(
        chipMint,
        userKeypair.publicKey,
        false,
        TOKEN_2022_PROGRAM_ID,
    );
    
    let instruction1 = createAssociatedTokenAccountIdempotentInstruction(
        userKeypair.publicKey,
        userChipAccount,
        userKeypair.publicKey,
        chipMint,
        TOKEN_2022_PROGRAM_ID,
    );

    // swap in 1000
    const amount = new BN(1_000_000_000);
    const txId = await program
        .methods
        .swap(true, amount)
        .preInstructions([instruction1])
        .accounts({
            user: userKeypair.publicKey,
            admin,
            adminAuthority,
            exchange,
            chipMint,
            exchangeCurrencyAccount,
            userCurrencyAccount,
            userChipAccount,
            tokenProgram: TOKEN_PROGRAM_ID,
            token2022Program: TOKEN_2022_PROGRAM_ID,
        })
        .signers([userKeypair])
        .rpc({
            skipPreflight: true,
            commitment: "finalized",
            maxRetries: 5,
        });
    console.log("swap in transaction id:", txId);
}

main().catch((err) => {
    console.error(err);
});
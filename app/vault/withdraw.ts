import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
    TOKEN_2022_PROGRAM_ID,
    getAssociatedTokenAddress,
    createAssociatedTokenAccountIdempotentInstruction
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
    const vault = new anchor.web3.PublicKey(Deployment.vault);
    // token accounts
    const lpTokenMint = new anchor.web3.PublicKey(Deployment.lpTokenMint);
    const chipMint = new anchor.web3.PublicKey(Deployment.chipMint);
    const vaultChipAccount = new anchor.web3.PublicKey(Deployment.vaultChipAccount);
    const userChipAccount = await getAssociatedTokenAddress(
        chipMint,
        userKeypair.publicKey,
        false,
        TOKEN_2022_PROGRAM_ID,
    );
    const userLpTokenAccount = await getAssociatedTokenAddress(
        lpTokenMint,
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

    // withdraw 100
    const amount = new BN(100_000_000);
    const txId = await program
        .methods
        .depositOrWithdraw(false, amount)
        .preInstructions([instruction1])
        .accounts({
            user: userKeypair.publicKey,
            admin,
            adminAuthority,
            vault,
            chipMint,
            lpTokenMint,
            vaultChipAccount,
            userChipAccount,
            userLpTokenAccount,
            tokenProgram: TOKEN_2022_PROGRAM_ID,
        })
        .signers([userKeypair])
        .rpc({
            skipPreflight: true,
            commitment: "finalized",
            maxRetries: 5,
        });
    console.log("withdraw transaction id:", txId);
}

main().catch((err) => {
    console.error(err);
});
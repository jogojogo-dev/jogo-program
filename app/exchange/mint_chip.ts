import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
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

    const operatorPrivateKey = bs58.decode(process.env.EXCHANGE_OPERATOR_PRIVATE_KEY || "");
    const operatorKeypair = anchor.web3.Keypair.fromSecretKey(operatorPrivateKey);
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
    const chipMint = new anchor.web3.PublicKey(Deployment.chipMint);
    const userChipAccount = await getAssociatedTokenAddress(
        chipMint,
        userKeypair.publicKey,
        false,
        TOKEN_2022_PROGRAM_ID,
    );

    let instruction1 = createAssociatedTokenAccountIdempotentInstruction(
        operatorKeypair.publicKey,
        userChipAccount,
        userKeypair.publicKey,
        chipMint,
        TOKEN_2022_PROGRAM_ID,
    );

    // mint 1000
    const amount = new BN(1_000_000_000);
    const txId = await program
        .methods
        .mintChip(amount)
        .preInstructions([instruction1])
        .accounts({
            operator: operatorKeypair.publicKey,
            user: userKeypair.publicKey,
            admin,
            adminAuthority,
            exchange,
            chipMint,
            userChipAccount,
            tokenProgram: TOKEN_2022_PROGRAM_ID,
        })
        .signers([operatorKeypair])
        .rpc({
            skipPreflight: true,
            commitment: "finalized",
            maxRetries: 5,
        });
    console.log("mint chip transaction id:", txId);
}

main().catch((err) => {
    console.error(err);
});
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import * as bs58 from "bs58";
import * as dotenv from "dotenv";
import { Buffer } from "buffer";
import {getAssociatedTokenAddress, TOKEN_2022_PROGRAM_ID} from "@solana/spl-token";
import { GameProgram } from "../../target/types/game_program";
import { Deployment } from "../deployment";
import BN from "bn.js";

dotenv.config();

async function main() {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.GameProgram as Program<GameProgram>;

    const userPrivateKey = bs58.decode(process.env.USER_PRIVATE_KEY || "");
    const userKeypair = anchor.web3.Keypair.fromSecretKey(userPrivateKey);
    const admin = new anchor.web3.PublicKey(Deployment.admin);
    const [game] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("game"), admin.toBuffer(), userKeypair.publicKey.toBuffer()],
        program.programId,
    );
    const [gameAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("authority"), game.toBuffer()],
        program.programId,
    );
    const tokenMint = new anchor.web3.PublicKey(Deployment.tokenMint);
    const supplyTokenAccount = await getAssociatedTokenAddress(
        tokenMint,
        gameAuthority,
        false,
        TOKEN_2022_PROGRAM_ID,
    );
    const userTokenAccount = await getAssociatedTokenAddress(
        tokenMint,
        userKeypair.publicKey,
        false,
        TOKEN_2022_PROGRAM_ID,
    );

    const amount = new BN(1_000_000);
    const txId = await program
        .methods
        .withdraw(amount)
        .accounts({
            owner: userKeypair.publicKey,
            game: game,
            authority: gameAuthority,
            tokenMint: tokenMint,
            ownerTokenAccount: userTokenAccount,
            supplyTokenAccount: supplyTokenAccount,
            tokenProgram: TOKEN_2022_PROGRAM_ID,
        })
        .signers([userKeypair])
        .rpc({
            skipPreflight: true,
            commitment: "confirmed",
            maxRetries: 5,
        });
    console.log("transaction id:", txId);
}

main().catch((err) => {
    console.error(err);
});
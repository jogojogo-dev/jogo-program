import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import * as bs58 from "bs58";
import * as dotenv from "dotenv";
import { Buffer } from "buffer";
import {getAssociatedTokenAddress, TOKEN_PROGRAM_ID} from "@solana/spl-token";
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
    const tokenMint = new anchor.web3.PublicKey(Deployment.tokenMint);
    const [game] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            Buffer.from("game"),
            admin.toBuffer(),
            userKeypair.publicKey.toBuffer(),
            tokenMint.toBuffer(),
        ],
        program.programId,
    );
    const userTokenAccount = await getAssociatedTokenAddress(
        tokenMint,
        userKeypair.publicKey,
        false,
        TOKEN_PROGRAM_ID,
    );
    const gameData = await program.account.game.fetch(game);

    const amount = new BN(100_000_000);
    const txId = await program
        .methods
        .deposit(amount)
        .accounts({
            owner: userKeypair.publicKey,
            game: game,
            tokenMint: tokenMint,
            ownerTokenAccount: userTokenAccount,
            supplyTokenAccount: gameData.supplyTokenAccount,
            tokenProgram: TOKEN_PROGRAM_ID,
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
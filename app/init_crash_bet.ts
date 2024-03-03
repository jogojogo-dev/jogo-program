import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { getAssociatedTokenAddress, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import * as bs58 from "bs58";
import * as dotenv from "dotenv";
import BN from "bn.js";
import { JogoProgram } from "../target/types/jogo_program";
import { Deployment } from "./deployment";

dotenv.config();

async function main() {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.JogoProgram as Program<JogoProgram>;

    const privateKey = bs58.decode(process.env.JOGO_OWNER_PRIVATE_KEY || "");
    const playerKeypair = anchor.web3.Keypair.fromSecretKey(privateKey);
    // global accounts
    const vault = new anchor.web3.PublicKey(Deployment.vault);
    // game accounts
    const game = new anchor.web3.PublicKey(Deployment.crashGame);
    const gameData = await program.account.crashGame.fetch(game);
    const [lock] = anchor.web3.PublicKey.findProgramAddressSync(
        [game.toBuffer(), gameData.nextRound.toBuffer("le")],
        program.programId,
    );
    const [bet] = anchor.web3.PublicKey.findProgramAddressSync(
        [lock.toBuffer(), playerKeypair.publicKey.toBuffer()],
        program.programId,
    );
    // token accounts
    const supplyTokenAccount = new anchor.web3.PublicKey(Deployment.supplyTokenAccount);
    const supplyTokenMint = new anchor.web3.PublicKey(Deployment.supplyToken);
    const playerTokenAccount = await getAssociatedTokenAddress(
        supplyTokenMint,
        playerKeypair.publicKey,
        false,
    );

    // stake 100
    const stakeAmount = new BN(100_000_000);
    const point = null;
    const txId = await program
        .methods
        .initCrashBet(stakeAmount, point)
        .accounts({
            player: playerKeypair.publicKey,
            vault: vault,
            game: game,
            lock: lock,
            bet: bet,
            supplyTokenAccount: supplyTokenAccount,
            playerTokenAccount: playerTokenAccount,
            tokenProgram: TOKEN_PROGRAM_ID,
            systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([playerKeypair])
        .rpc({
            skipPreflight: true,
            commitment: "finalized",
            maxRetries: 5,
        });
    console.log("transaction id:", txId);
}

main().catch((err) => {
    console.error(err);
});
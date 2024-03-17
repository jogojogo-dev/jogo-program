import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { getAssociatedTokenAddress, TOKEN_2022_PROGRAM_ID } from "@solana/spl-token";
import * as bs58 from "bs58";
import * as dotenv from "dotenv";
import BN from "bn.js";
import { JogoProgram } from "../../target/types/jogo_program";
import { Deployment } from "../deployment";

dotenv.config();

async function main() {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.JogoProgram as Program<JogoProgram>;

    const userPrivateKey = bs58.decode(process.env.USER_PRIVATE_KEY || "");
    const playerKeypair = anchor.web3.Keypair.fromSecretKey(userPrivateKey);
    // global accounts
    const vault = new anchor.web3.PublicKey(Deployment.vault);
    // game accounts
    const game = new anchor.web3.PublicKey(Deployment.crashGame);
    const gameData = await program.account.crashGame.fetch(game);
    const [lock] = anchor.web3.PublicKey.findProgramAddressSync(
        [game.toBuffer(), gameData.nextRound.toBuffer("le", 8)],
        program.programId,
    );
    const [bet] = anchor.web3.PublicKey.findProgramAddressSync(
        [lock.toBuffer(), playerKeypair.publicKey.toBuffer()],
        program.programId,
    );
    // token accounts
    const chipMint = new anchor.web3.PublicKey(Deployment.chipMint);
    const vaultChipAccount = new anchor.web3.PublicKey(Deployment.vaultChipAccount);
    const playerChipAccount = await getAssociatedTokenAddress(
        chipMint,
        playerKeypair.publicKey,
        false,
    );

    // stake 1
    const stakeAmount = new BN(1_000_000);
    // not preset point
    const point = null;
    const txId = await program
        .methods
        .createCrashBet(stakeAmount, point)
        .accounts({
            player: playerKeypair.publicKey,
            vault,
            game,
            lock,
            bet,
            chipMint,
            vaultChipAccount,
            playerChipAccount,
            tokenProgram: TOKEN_2022_PROGRAM_ID,
            systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([playerKeypair])
        .rpc({
            skipPreflight: false,
            commitment: "confirmed",
            maxRetries: 5,
        });
    console.log("transaction id:", txId);
}

main().catch((err) => {
    console.error(err);
});
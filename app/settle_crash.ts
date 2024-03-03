import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { getAssociatedTokenAddress, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { ed25519 } from "@noble/curves/ed25519";
import * as bs58 from "bs58";
import * as dotenv from "dotenv";
import BN from "bn.js";
import { Buffer } from "buffer";
import { JogoProgram } from "../target/types/jogo_program";
import { Deployment } from "./deployment";
import { packBetMessage, Fraction } from "./utils";

dotenv.config();

async function main() {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.JogoProgram as Program<JogoProgram>;

    const operatorPrivateKey = bs58.decode(process.env.CRASH_OPERATOR_PRIVATE_KEY || "").slice(0, 32);
    const playerPrivateKey = bs58.decode(process.env.USER_PRIVATE_KEY || "");
    const playerKeypair = anchor.web3.Keypair.fromSecretKey(playerPrivateKey);

    // global accounts
    const admin = new anchor.web3.PublicKey(Deployment.admin);
    const [adminAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("authority"), admin.toBuffer()],
        program.programId,
    );
    const vault = new anchor.web3.PublicKey(Deployment.vault);
    // game accounts
    const game = new anchor.web3.PublicKey(Deployment.crashGame);
    const gameRound = new BN(0);
    const [lock] = anchor.web3.PublicKey.findProgramAddressSync(
        [game.toBuffer(), gameRound.toBuffer("le", 8)],
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

    // prepare instruction data
    const lockData = await program.account.crashLock.fetch(lock);
    const randomnessSig = ed25519.sign(Uint8Array.from(lockData.randomness), operatorPrivateKey);
    // player point 1.5
    const point = Fraction.fromNumber(3, 2);
    const betMessage = packBetMessage(bet.toBytes(), point);
    const betSig = ed25519.sign(betMessage, operatorPrivateKey);

    const txId = await program
        .methods
        .settleCrash(Array.from(randomnessSig), Array.from(betSig), point)
        .accounts({
            player: playerKeypair.publicKey,
            admin: admin,
            adminAuthority: adminAuthority,
            vault: vault,
            game: game,
            lock: lock,
            bet: bet,
            supplyTokenAccount: supplyTokenAccount,
            playerTokenAccount: playerTokenAccount,
            tokenProgram: TOKEN_PROGRAM_ID,
            ed25519Program: anchor.web3.Ed25519Program.programId,
            systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([playerKeypair])
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
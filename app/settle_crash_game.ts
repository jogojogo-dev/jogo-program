import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { getAssociatedTokenAddress, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import * as bs58 from "bs58";
import * as dotenv from "dotenv";
import BN from "bn.js";
import { Buffer } from "buffer";
import { JogoProgram } from "../target/types/jogo_program";
import { Deployment } from "./deployment";
import { packBetMessage, pointNumberToBN } from "./utils";

dotenv.config();

async function main() {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.JogoProgram as Program<JogoProgram>;

    const operatorPrivateKey = bs58.decode(process.env.CRASH_OPERATOR_PRIVATE_KEY || "");
    const operatorKeypair = anchor.web3.Keypair.fromSecretKey(operatorPrivateKey);
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

    const lockData = await program.account.crashLock.fetch(lock);
    // prepare instruction data
    const randomness = new Uint8Array(lockData.randomness);
    const instruction1 = anchor.web3.Ed25519Program.createInstructionWithPrivateKey({
        privateKey: operatorKeypair.secretKey.slice(0, 32),
        message: randomness,
    });

    // player point 1.5
    const point = pointNumberToBN(1.5);
    const betMessage = packBetMessage(bet.toBytes(), point);
    const instruction2 = anchor.web3.Ed25519Program.createInstructionWithPrivateKey({
        privateKey: operatorKeypair.secretKey.slice(0, 32),
        message: betMessage,
    });

    const txId = await program
        .methods
        .settleCrash()
        .preInstructions([instruction1, instruction2])
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
            instructions: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
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
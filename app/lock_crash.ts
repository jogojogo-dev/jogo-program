import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { randomnessAccountAddress } from "@orao-network/solana-vrf"
import * as bs58 from "bs58";
import * as dotenv from "dotenv";
import { JogoProgram } from "../target/types/jogo_program";
import { Deployment } from "./deployment";
import { randomSeed } from "./utils";

dotenv.config();

async function main() {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.JogoProgram as Program<JogoProgram>;

    const privateKey = bs58.decode(process.env.CRASH_OPERATOR_PRIVATE_KEY || "");
    const operatorKeypair = anchor.web3.Keypair.fromSecretKey(privateKey);
    // game accounts
    const game = new anchor.web3.PublicKey(Deployment.crashGame);
    const gameData = await program.account.crashGame.fetch(game);
    const [lock] = anchor.web3.PublicKey.findProgramAddressSync(
        [game.toBuffer(), gameData.nextRound.toBuffer("le", 8)],
        program.programId,
    );
    const lockData = await program.account.crashLock.fetch(lock);
    const [lastLock] = anchor.web3.PublicKey.findProgramAddressSync(
        [game.toBuffer(), lockData.round.toBuffer("le", 8)],
        program.programId,
    );

    // vrf accounts
    const seed = randomSeed(lock.toBytes(), new Uint8Array(gameData.lastRandomness))
    const randomness = randomnessAccountAddress(seed);
    
    const txId = await program
        .methods
        .lockCrash()
        .accounts({
            operator: operatorKeypair.publicKey,
            game: game,
            lastLock: lastLock,
            lock: lock,
            randomness: randomness,
            systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([operatorKeypair])
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
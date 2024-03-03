import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { randomnessAccountAddress } from "@orao-network/solana-vrf"
import * as bs58 from "bs58";
import * as dotenv from "dotenv";
import { JogoProgram } from "../target/types/jogo_program";
import { Deployment } from "./deployment";

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
    // vrf accounts
    const randomness = randomnessAccountAddress(lock.toBuffer());

    const txId = await program
        .methods
        .lockCrashBet()
        .accounts({
            operator: operatorKeypair.publicKey,
            game: game,
            lock: lock,
            randomness: randomness,
            systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([operatorKeypair])
        .rpc({
            skipPreflight: true,
            commitment: "processed",
            maxRetries: 5,
        });
    console.log("transaction id:", txId);
}

main().catch((err) => {
    console.error(err);
});
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Orao } from "@orao-network/solana-vrf"
import { ed25519 } from "@noble/curves/ed25519";
import * as bs58 from "bs58";
import * as dotenv from "dotenv";
import { JogoProgram } from "../target/types/jogo_program";
import { Deployment } from "./deployment";
import { randomSeed, computeCrashPoint, Fraction } from "./utils";

dotenv.config();

async function main() {
    // Configure the client
    anchor.setProvider(anchor.AnchorProvider.env());

    const jogo_program = anchor.workspace.JogoProgram as Program<JogoProgram>;
    const vrf_program = new Orao(anchor.AnchorProvider.env());

    const game = new anchor.web3.PublicKey(Deployment.crashGame);
    const gameData = await jogo_program.account.crashGame.fetch(game);
    const [lock] = anchor.web3.PublicKey.findProgramAddressSync(
        [game.toBuffer(), gameData.nextRound.toBuffer("le", 8)],
        jogo_program.programId,
    );
    const seed = randomSeed(lock.toBytes(), Uint8Array.from(gameData.lastRandomness));

    const builder = await vrf_program.request(seed);
    const [, tx] = await builder.rpc()
    console.log("transaction id:", tx);

    // Await fulfilled randomness (default commitment is "finalized"):
    const randomness = await vrf_program.waitFulfilled(seed, "confirmed");
    // Show the final crash point
    const operatorPrivateKey = bs58.decode(process.env.CRASH_OPERATOR_PRIVATE_KEY || "");
    const randomnessSig = ed25519.sign(randomness.randomness, operatorPrivateKey);
    const crashPoint = computeCrashPoint(randomnessSig, Fraction.fromJson(gameData.winRate));
    console.log("crash point:", crashPoint.toFloat());
}

main().catch((err) => {
    console.error(err);
});
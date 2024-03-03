import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Orao } from "@orao-network/solana-vrf"
import * as dotenv from "dotenv";
import { JogoProgram } from "../target/types/jogo_program";
import { Deployment } from "./deployment";
import BN from "bn.js";

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
    const seed = lock.toBuffer();

    const builder = await vrf_program.request(seed);
    const [, tx] = await builder.rpc()
    console.log("transaction id:", tx);

    // Await fulfilled randomness (default commitment is "finalized"):
    const randomness = await vrf_program.waitFulfilled(seed);
    console.log("randomness is " + randomness.fulfilled().toString());
}

main().catch((err) => {
    console.error(err);
});
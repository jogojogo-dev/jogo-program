import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import * as bs58 from "bs58";
import * as dotenv from "dotenv";
import { JogoProgram } from "../target/types/jogo_program";
import { Deployment } from "./deployment";
import { Fraction } from "./utils";

dotenv.config();

async function main() {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.JogoProgram as Program<JogoProgram>;

    const ownerPrivateKey = bs58.decode(process.env.JOGO_OWNER_PRIVATE_KEY || "");
    const ownerKeypair = anchor.web3.Keypair.fromSecretKey(ownerPrivateKey);
    const admin = new anchor.web3.PublicKey(Deployment.admin);
    const vault = new anchor.web3.PublicKey(Deployment.vault);
    const gameKeypair = anchor.web3.Keypair.generate();

    const operatorPrivateKey = bs58.decode(process.env.CRASH_OPERATOR_PRIVATE_KEY || "");
    const operatorKeypair = anchor.web3.Keypair.fromSecretKey(operatorPrivateKey);
    // 95% win rate
    const win_rate = Fraction.fromNumber(95, 100);
    // max odd 10
    const max_odd = Fraction.fromNumber(10, 1);
    const txId = await program
        .methods
        .initCrashGame(operatorKeypair.publicKey, win_rate.toJson(), max_odd.toJson())
        .accounts({
            owner: ownerKeypair.publicKey,
            admin: admin,
            vault: vault,
            game: gameKeypair.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([ownerKeypair, gameKeypair])
        .rpc({
            skipPreflight: true,
            commitment: "confirmed",
            maxRetries: 5,
        });
    console.log("transaction id:", txId);
    console.log("crash game:", gameKeypair.publicKey.toString());
}

main().catch((err) => {
    console.error(err);
});
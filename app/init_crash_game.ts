import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
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
    const ownerKeypair = anchor.web3.Keypair.fromSecretKey(privateKey);
    const admin = new anchor.web3.PublicKey(Deployment.admin);
    const vault = new anchor.web3.PublicKey(Deployment.vault);
    const gameKeypair = anchor.web3.Keypair.generate();

    const operator = ownerKeypair.publicKey;
    // 95% win rate
    const win_rate = {
        numerator: new BN(95),
        denominator: new BN(100),
    };
    // max odd 10
    const max_odd = {
        numerator: new BN(10),
        denominator: new BN(1),
    }
    
    const txId = await program
        .methods
        .initCrashGame(operator, win_rate, max_odd)
        .accounts({
            owner: ownerKeypair.publicKey,
            admin: admin,
            vault: vault,
            game: gameKeypair.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([ownerKeypair])
        .rpc({
            skipPreflight: true,
            commitment: "finalized",
            maxRetries: 5,
        });
    console.log("transaction id:", txId);
    console.log("crash game:", gameKeypair.publicKey);
}

main().catch((err) => {
    console.error(err);
});
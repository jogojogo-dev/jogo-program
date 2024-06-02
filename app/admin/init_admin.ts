import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import * as bs58 from "bs58";
import * as dotenv from "dotenv";
import { Buffer } from "buffer";
import { GameProgram } from "../../target/types/game_program";

dotenv.config();

async function main() {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.GameProgram as Program<GameProgram>;

    const ownerPrivateKey = bs58.decode(process.env.OWNER_PRIVATE_KEY || "");
    const ownerKeypair = anchor.web3.Keypair.fromSecretKey(ownerPrivateKey);
    const adminKeypair = anchor.web3.Keypair.generate();

    const txId = await program
        .methods
        .initAdmin()
        .accounts({
            owner: ownerKeypair.publicKey,
            admin: adminKeypair.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([ownerKeypair, adminKeypair])
        .rpc({
            skipPreflight: true,
            commitment: "confirmed",
            maxRetries: 5,
        });
    console.log("transaction id:", txId);
    console.log("admin:", adminKeypair.publicKey.toString());
}

main().catch((err) => {
    console.error(err);
});
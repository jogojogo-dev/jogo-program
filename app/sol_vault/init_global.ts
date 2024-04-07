import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import * as bs58 from "bs58";
import * as dotenv from "dotenv";
import {Buffer } from "buffer";
import { VaultProgram } from "../../target/types/vault_program";

dotenv.config();

async function main() {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.VaultProgram as Program<VaultProgram>;

    const payerPrivateKey = bs58.decode(process.env.OWNER_PRIVATE_KEY || "");
    const payerKeypair = anchor.web3.Keypair.fromSecretKey(payerPrivateKey);
    const [global] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("global")],
        program.programId,
    );

    const txId = await program
        .methods
        .initGlobal()
        .accounts({
            payer: payerKeypair.publicKey,
            global,
            systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([payerKeypair])
        .rpc({
            skipPreflight: true,
            commitment: "confirmed",
            maxRetries: 5,
        });
    console.log("transaction id:", txId);
    console.log("global:", global.toString());
}

main().catch((err) => {
    console.error(err);
});
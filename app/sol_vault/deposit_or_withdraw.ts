import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import * as bs58 from "bs58";
import * as dotenv from "dotenv";
import { Buffer } from "buffer";
import BN from "bn.js";
import { VaultProgram } from "../../target/types/vault_program";

dotenv.config();

async function main() {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.VaultProgram as Program<VaultProgram>;

    const userPrivateKey = bs58.decode(process.env.USER_PRIVATE_KEY || "");
    const userKeypair = anchor.web3.Keypair.fromSecretKey(userPrivateKey);
    const [global] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("global")],
        program.programId,
    );
    const [credential] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("credential"), userKeypair.publicKey.toBuffer()],
        program.programId,
    );

    const isDeposit = true;
    const amount = new BN(1_000_000_000);
    const txId = await program
        .methods
        .depositOrWithdraw(isDeposit, amount)
        .accounts({
            user: userKeypair.publicKey,
            global,
            credential,
            systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([userKeypair])
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
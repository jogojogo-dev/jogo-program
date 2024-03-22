import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import * as bs58 from "bs58";
import * as dotenv from "dotenv";
import { JogoProgram } from "../../target/types/jogo_program";
import { Deployment } from "../deployment";

dotenv.config();

async function main() {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.JogoProgram as Program<JogoProgram>;

    const ownerPrivateKey = bs58.decode(process.env.OWNER_PRIVATE_KEY || "");
    const ownerKeypair = anchor.web3.Keypair.fromSecretKey(ownerPrivateKey);
    const operatorPrivateKey = bs58.decode(process.env.CRASH_OPERATOR_PRIVATE_KEY || "");
    const operatorKeypair = anchor.web3.Keypair.fromSecretKey(operatorPrivateKey);
    const admin = new anchor.web3.PublicKey(Deployment.admin);
    const vault = new anchor.web3.PublicKey(Deployment.vault);
    const thirdPartyGameKeypair = anchor.web3.Keypair.generate();

    const txId = await program
        .methods
        .initThirdPartyGame(operatorKeypair.publicKey)
        .accounts({
            owner: ownerKeypair.publicKey,
            admin,
            vault,
            thirdParty: thirdPartyGameKeypair.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([ownerKeypair, thirdPartyGameKeypair])
        .rpc({
            skipPreflight: true,
            commitment: "confirmed",
            maxRetries: 5,
        });
    console.log("transaction id:", txId);
    console.log("third party game:", thirdPartyGameKeypair.publicKey.toString());
}

main().catch((err) => {
    console.error(err);
});
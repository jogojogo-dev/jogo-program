import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import * as bs58 from "bs58";
import * as dotenv from "dotenv";
import { Buffer } from "buffer";
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
    const [adminAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            Buffer.from("authority"),
            admin.toBuffer(),
        ],
        program.programId,
    );
    const vaultKeypair = anchor.web3.Keypair.generate();
    const supplyTokenMint = new anchor.web3.PublicKey(Deployment.supplyToken);
    const supplyTokenAccountKeypair = anchor.web3.Keypair.generate();
    const lpTokenMintKeypair = anchor.web3.Keypair.generate();

    const txId = await program
        .methods
        .initVault()
        .accounts({
            owner: ownerKeypair.publicKey,
            admin: admin,
            adminAuthority: adminAuthority,
            vault: vaultKeypair.publicKey,
            supplyTokenMint: supplyTokenMint,
            supplyTokenAccount: supplyTokenAccountKeypair.publicKey,
            lpTokenMint: lpTokenMintKeypair.publicKey,
            tokenProgram: TOKEN_PROGRAM_ID,
            systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([ownerKeypair, vaultKeypair, supplyTokenAccountKeypair, lpTokenMintKeypair])
        .rpc({
            skipPreflight: true,
            commitment: "confirmed",
            maxRetries: 5,
        });
    console.log("transaction id:", txId);
    console.log("vault:", vaultKeypair.publicKey.toString());
}

main().catch((err) => {
    console.error(err);
});
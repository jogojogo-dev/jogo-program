import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import * as bs58 from "bs58";
import * as dotenv from "dotenv";
import { Buffer } from "buffer";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { GameProgram } from "../../target/types/game_program";
import { Deployment } from "../deployment";

dotenv.config();

async function main() {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.GameProgram as Program<GameProgram>;

    const ownerPrivateKey = bs58.decode(process.env.OWNER_PRIVATE_KEY || "");
    const ownerKeypair = anchor.web3.Keypair.fromSecretKey(ownerPrivateKey);
    const admin = new anchor.web3.PublicKey(Deployment.admin);
    const tokenMint = new anchor.web3.PublicKey(Deployment.tokenMint);
    const identifier = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    const [club] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            Buffer.from("club"),
            admin.toBuffer(),
            ownerKeypair.publicKey.toBuffer(),
            tokenMint.toBuffer(),
            Buffer.from(identifier),
        ],
        program.programId,
    );
    const [clubAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("authority"), club.toBuffer()],
        program.programId,
    );
    const supplyTokenAccountKeypair = anchor.web3.Keypair.generate();

    const txId = await program
        .methods
        .initClub(identifier)
        .accounts({
            owner: ownerKeypair.publicKey,
            admin: admin,
            club: club,
            clubAuthority: clubAuthority,
            supplyTokenMint: tokenMint,
            supplyTokenAccount: supplyTokenAccountKeypair.publicKey,
            tokenProgram: TOKEN_PROGRAM_ID,
            systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([ownerKeypair, supplyTokenAccountKeypair])
        .rpc({
            skipPreflight: true,
            commitment: "confirmed",
            maxRetries: 5,
        });
    console.log("transaction id:", txId);
    console.log("club:", club.toString())
}

main().catch((err) => {
    console.error(err);
});
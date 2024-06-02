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

    const userPrivateKey = bs58.decode(process.env.USER_PRIVATE_KEY || "");
    const userKeypair = anchor.web3.Keypair.fromSecretKey(userPrivateKey);
    const admin = new anchor.web3.PublicKey(Deployment.admin);
    const tokenMint = new anchor.web3.PublicKey(Deployment.tokenMint);
    const [game] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            Buffer.from("game"),
            admin.toBuffer(),
            userKeypair.publicKey.toBuffer(),
            tokenMint.toBuffer(),
        ],
        program.programId,
    );
    const [gameAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("authority"), game.toBuffer()],
        program.programId,
    );
    const supplyTokenAccountKeypair = anchor.web3.Keypair.generate();

    const txId = await program
        .methods
        .initGame()
        .accounts({
            owner: userKeypair.publicKey,
            admin: admin,
            game: game,
            authority: gameAuthority,
            supplyTokenMint: tokenMint,
            supplyTokenAccount: supplyTokenAccountKeypair.publicKey,
            tokenProgram: TOKEN_PROGRAM_ID,
            systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([userKeypair, supplyTokenAccountKeypair])
        .rpc({
            skipPreflight: true,
            commitment: "confirmed",
            maxRetries: 5,
        });
    console.log("transaction id:", txId);
    console.log("game:", game.toString())
}

main().catch((err) => {
    console.error(err);
});
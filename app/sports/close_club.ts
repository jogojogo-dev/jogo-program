import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import * as bs58 from "bs58";
import * as dotenv from "dotenv";
import { Buffer } from "buffer";
import {ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddress, TOKEN_PROGRAM_ID} from "@solana/spl-token";
import { SportsProgram } from "../../target/types/sports_program";
import { Deployment } from "../deployment";

dotenv.config();

async function main() {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.SportsProgram as Program<SportsProgram>;

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
    const supplyTokenAccount = await getAssociatedTokenAddress(
        tokenMint,
        clubAuthority,
        true,
        TOKEN_PROGRAM_ID,
    );
    const ownerTokenAccount = await getAssociatedTokenAddress(
        tokenMint,
        ownerKeypair.publicKey,
        false,
        TOKEN_PROGRAM_ID,
    );

    const txId = await program
        .methods
        .closeClub()
        .accounts({
            owner: ownerKeypair.publicKey,
            club: club,
            clubAuthority: clubAuthority,
            tokenMint: tokenMint,
            ownerTokenAccount: ownerTokenAccount,
            supplyTokenAccount: supplyTokenAccount,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([ownerKeypair])
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
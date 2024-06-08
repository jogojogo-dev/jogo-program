import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import * as bs58 from "bs58";
import * as dotenv from "dotenv";
import { Buffer } from "buffer";
import {ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddress, TOKEN_PROGRAM_ID} from "@solana/spl-token";
import { SportsProgram } from "../../target/types/sports_program";
import { Deployment } from "../deployment";
import BN from "bn.js";

dotenv.config();

async function main() {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.SportsProgram as Program<SportsProgram>;

    const ownerPrivateKey = bs58.decode(process.env.OWNER_PRIVATE_KEY || "");
    const ownerKeypair = anchor.web3.Keypair.fromSecretKey(ownerPrivateKey);
    const userPrivateKey = bs58.decode(process.env.USER_PRIVATE_KEY || "");
    const userKeypair = anchor.web3.Keypair.fromSecretKey(userPrivateKey);
    const operatorPrivateKey = bs58.decode(process.env.OPERATOR_PRIVATE_KEY || "");
    const operatorKeypair = anchor.web3.Keypair.fromSecretKey(operatorPrivateKey);
    const admin = new anchor.web3.PublicKey(Deployment.admin);
    const tokenMint = new anchor.web3.PublicKey(Deployment.tokenMint);
    const identifier1 = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    const [club] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            Buffer.from("club"),
            admin.toBuffer(),
            ownerKeypair.publicKey.toBuffer(),
            tokenMint.toBuffer(),
            Buffer.from(identifier1),
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

    const identifier2 = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    const [credential] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            Buffer.from("credential"),
            club.toBuffer(),
            userKeypair.publicKey.toBuffer(),
            Buffer.from(identifier2),
        ],
        program.programId,
    );
    const userTokenAccount = await getAssociatedTokenAddress(
        tokenMint,
        userKeypair.publicKey,
        false,
        TOKEN_PROGRAM_ID,
    );

    const direction = 0;
    const stake = new BN(1_000_000);
    const lock = new BN(5_000_000);
    const txId = await program
        .methods
        .bet(identifier2, direction, stake, lock)
        .accounts({
            player: userKeypair.publicKey,
            operator: operatorKeypair.publicKey,
            admin: admin,
            club: club,
            clubAuthority: clubAuthority,
            credential: credential,
            tokenMint: tokenMint,
            playerTokenAccount: userTokenAccount,
            supplyTokenAccount: supplyTokenAccount,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            tokenProgram: TOKEN_PROGRAM_ID,
            systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([operatorKeypair, userKeypair])
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
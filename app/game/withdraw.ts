import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import * as bs58 from "bs58";
import * as dotenv from "dotenv";
import { Buffer } from "buffer";
import {getAssociatedTokenAddress, TOKEN_PROGRAM_ID} from "@solana/spl-token";
import { GameProgram } from "../../target/types/game_program";
import { Deployment } from "../deployment";
import BN from "bn.js";

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
    const clubData = await program.account.club.fetch(club);
    const ownerTokenAccount = await getAssociatedTokenAddress(
        tokenMint,
        ownerKeypair.publicKey,
        false,
        TOKEN_PROGRAM_ID,
    );

    const amount = new BN(1_000_000);
    const txId = await program
        .methods
        .withdraw(amount)
        .accounts({
            owner: ownerKeypair.publicKey,
            club: club,
            clubAuthority: clubAuthority,
            tokenMint: tokenMint,
            ownerTokenAccount: ownerTokenAccount,
            supplyTokenAccount: clubData.supplyTokenAccount,
            tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([ownerKeypair])
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
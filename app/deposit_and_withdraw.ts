import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TOKEN_2022_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddress } from "@solana/spl-token";
import * as bs58 from "bs58";
import * as dotenv from "dotenv";
import { Buffer } from "buffer";
import BN from "bn.js";
import { JogoProgram } from "../target/types/jogo_program";
import { Deployment } from "./deployment";

dotenv.config();

async function main() {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.JogoProgram as Program<JogoProgram>;

    const privateKey = bs58.decode(process.env.USER_PRIVATE_KEY || "");
    const userKeypair = anchor.web3.Keypair.fromSecretKey(privateKey);
    // global accounts
    const admin = new anchor.web3.PublicKey(Deployment.admin);
    const [adminAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("authority"), admin.toBuffer()],
        program.programId,
    );
    const vault = new anchor.web3.PublicKey(Deployment.vault);
    // token accounts
    const lpTokenMint = new anchor.web3.PublicKey(Deployment.lpTokenMint);
    const chipMint = new anchor.web3.PublicKey(Deployment.chipMint);
    const supplyChipAccount = new anchor.web3.PublicKey(Deployment.supplyChipAccount);
    const userChipAccount = await getAssociatedTokenAddress(
        chipMint,
        userKeypair.publicKey,
        false,
    );
    const userLpTokenAccount = await getAssociatedTokenAddress(
        lpTokenMint,
        userKeypair.publicKey,
        false,
    );

    // deposit 1000
    const amount = new BN(1_000_000_000);
    const txId = await program
        .methods
        .deposit(amount)
        .accounts({
            user: userKeypair.publicKey,
            admin,
            adminAuthority,
            vault,
            chipMint,
            lpTokenMint,
            supplyChipAccount,
            userChipAccount,
            userLpTokenAccount,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            tokenProgram: TOKEN_2022_PROGRAM_ID,
            systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([userKeypair])
        .rpc({
            skipPreflight: true,
            commitment: "finalized",
            maxRetries: 5,
        });
    console.log("deposit transaction id:", txId);

    // withdraw 100 LP
    const lpAmount = new BN(100);
    const withdrawTxId = await program
        .methods
        .withdraw(lpAmount)
        .accounts({
            user: userKeypair.publicKey,
            admin,
            adminAuthority,
            vault,
            lpTokenMint,
            supplyChipAccount,
            userChipAccount,
            userLpTokenAccount,
            tokenProgram: TOKEN_2022_PROGRAM_ID,
            systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([userKeypair])
        .rpc({
            skipPreflight: true,
            commitment: "finalized",
            maxRetries: 5,
        });
    console.log("withdraw transaction id:", withdrawTxId);
}

main().catch((err) => {
    console.error(err);
});
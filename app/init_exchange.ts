import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TOKEN_2022_PROGRAM_ID, TOKEN_PROGRAM_ID } from "@solana/spl-token";
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

    const ownerPrivateKey = bs58.decode(process.env.JOGO_OWNER_PRIVATE_KEY || "");
    const ownerKeypair = anchor.web3.Keypair.fromSecretKey(ownerPrivateKey);
    const operatorPrivateKey = bs58.decode(process.env.CRASH_OPERATOR_PRIVATE_KEY || "");
    const operatorKeypair = anchor.web3.Keypair.fromSecretKey(operatorPrivateKey);
    const admin = new anchor.web3.PublicKey(Deployment.admin);
    const [adminAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            Buffer.from("authority"),
            admin.toBuffer(),
        ],
        program.programId,
    );
    const exchangeKeypair = anchor.web3.Keypair.generate();
    const currencyMint = new anchor.web3.PublicKey(Deployment.currencyMint);
    const currencyAccountKeypair = anchor.web3.Keypair.generate();
    const chipMintKeypair = anchor.web3.Keypair.generate();

    const txId = await program
        .methods
        .initExchange(operatorKeypair.publicKey)
        .accounts({
            owner: ownerKeypair.publicKey,
            admin,
            adminAuthority,
            exchange: exchangeKeypair.publicKey,
            currencyMint,
            exchangeCurrencyAccount: currencyAccountKeypair.publicKey,
            chipMint: chipMintKeypair.publicKey,
            tokenProgram: TOKEN_PROGRAM_ID,
            token2022Program: TOKEN_2022_PROGRAM_ID,
            systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([ownerKeypair, exchangeKeypair, currencyAccountKeypair, chipMintKeypair])
        .rpc({
            skipPreflight: true,
            commitment: "confirmed",
            maxRetries: 5,
        });
    console.log("transaction id:", txId);
    console.log("exchange:", exchangeKeypair.publicKey.toString());
    console.log("currency account:", currencyAccountKeypair.publicKey.toString());
    console.log("chip mint:", chipMintKeypair.publicKey.toString());
}

main().catch((err) => {
    console.error(err);
});
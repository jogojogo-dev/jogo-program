import { Program } from "@coral-xyz/anchor";
import * as anchor from "@coral-xyz/anchor";
import { Buffer } from "buffer";
import * as dotenv from "dotenv";
import { VaultProgram } from "../../target/types/vault_program";
import * as bs58 from "bs58";

dotenv.config();

async function getTVL(program: Program<VaultProgram>) {
    const [global] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("global")],
        program.programId,
    );
    const globalData = await program.account.global.fetch(global);
    console.log("total tvl amount:", globalData.amount.toNumber());
}

async function getUserAmount(program: Program<VaultProgram>) {
    const userPrivateKey = bs58.decode(process.env.USER_PRIVATE_KEY || "");
    const userKeypair = anchor.web3.Keypair.fromSecretKey(userPrivateKey);
    const [credential] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("credential"), userKeypair.publicKey.toBuffer()],
        program.programId,
    );

    try {
        const credentialData = await program.account.credential.fetch(credential);
        console.log("user amount:", credentialData.amount.toNumber());
    } catch (err) {
        console.log("user has no amount.");
    }
}

async function main() {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    // Configure the client to use the local cluster.
    const program = anchor.workspace.VaultProgram as Program<VaultProgram>;

    await getTVL(program);
    await getUserAmount(program);
}

main().catch((err) => {
    console.error(err);
});
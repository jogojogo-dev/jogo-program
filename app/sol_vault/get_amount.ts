import { Program } from "@coral-xyz/anchor";
import * as anchor from "@coral-xyz/anchor";
import { Buffer } from "buffer";
import { VaultProgram } from "../../target/types/vault_program";

async function getTVL(program: Program<VaultProgram>) {
    const [global] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("global")],
        program.programId,
    );
    const globalData = await program.account.global.fetch(global);
    console.log("total tvl amount:", globalData.amount);
}

async function getUserAmount(program: Program<VaultProgram>) {
    const userPrivateKey = Buffer.from(process.env.USER_PRIVATE_KEY || "", "base64");
    const userKeypair = anchor.web3.Keypair.fromSecretKey(userPrivateKey);
    const [credential] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("credential"), userKeypair.publicKey.toBuffer()],
        program.programId,
    );
    const credentialData = await program.account.credential.fetch(credential);
    console.log("user amount:", credentialData.amount);
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
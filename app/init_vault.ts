import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { JogoProgram } from "../target/types/jogo_program";
import * as bs58 from "bs58";

async function main() {

    const keypair = anchor.web3.Keypair.generate();
    console.log(keypair.secretKey);
    console.log(keypair.publicKey.toBase58())

    // Configure the client to use the local cluster.
    // anchor.setProvider(anchor.AnchorProvider.local());


}

main().catch((err) => {
    console.error(err);
});
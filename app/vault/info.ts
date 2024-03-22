import { Program } from "@coral-xyz/anchor";
import { JogoProgram } from "../../target/types/jogo_program";
import { Deployment } from "../deployment";
import * as anchor from "@coral-xyz/anchor";

async function getVaultInfo(program: Program<JogoProgram>) {
    const vaultData = await program.account.vault.fetch(Deployment.vault);

    const liquidity = vaultData.liquidity.toNumber();
    console.log("vault liquidity:", liquidity);

    const marketCap = vaultData.liquidity.add(vaultData.reserve).sub(vaultData.stake).toNumber();
    console.log("market cap:", marketCap);

    const lpSupply = vaultData.mintedLp.toNumber();
    console.log("lp supply:", lpSupply);

    const lpPrice = marketCap / lpSupply;
    console.log("lp price:", lpPrice);
}

async function main() {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    // Configure the client to use the local cluster.
    const program = anchor.workspace.JogoProgram as Program<JogoProgram>;

    await getVaultInfo(program);
}

main().catch((err) => {
    console.error(err);
});
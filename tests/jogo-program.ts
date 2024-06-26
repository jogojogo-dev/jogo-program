import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { JogoProgram } from "../target/types/jogo_program";

describe("jogo-program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.JogoProgram as Program<JogoProgram>;

  it("Init Admin", async () => {
    // Add your test here.
    const tx = await program.methods.initAdmin().rpc();
    console.log("Your transaction signature", tx);
  });
});

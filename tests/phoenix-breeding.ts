import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { PhoenixBreeding } from "../target/types/phoenix_breeding";

describe("phoenix-breeding", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.PhoenixBreeding as Program<PhoenixBreeding>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});

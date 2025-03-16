import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Splitlana } from "../target/types/splitlana";
import { randomBytes } from "crypto";

describe("splitlana", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.splitlana as Program<Splitlana>;

  it("Is initialized!", async () => {
    // Add your test here.
    const seed = new anchor.BN(randomBytes(8));
    const tx = await program.methods.initBill(seed, new anchor.BN(10), "cenas", {sol: {}}).accounts({
      user: provider.publicKey,
    })
    .rpc();
    console.log(tx);
  });
});

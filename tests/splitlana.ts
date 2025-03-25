import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Splitlana } from "../target/types/splitlana";
import { randomBytes } from "crypto";

describe("splitlana", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.splitlana as Program<Splitlana>;

  const payerAkeypair = anchor.web3.Keypair.generate();

  const seedBill = new anchor.BN(randomBytes(8));
  const seedBillBytes = seedBill.toArrayLike(Buffer, "le", 8);
  const billPda = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("bill"), provider.publicKey.toBuffer(), seedBillBytes],
    program.programId
  )[0];

  it("Initialize Bill", async () => {
    // Add your test here.
    const tx = await program.methods
    .initBill(seedBill, new anchor.BN(10), "Kill Bill", {sol: {}})
    .accountsPartial({
      user: provider.publicKey,
      bill: billPda,
    })
    .rpc();
    console.log(tx);
  });

  it("Add payer to existing bill", async () => {
    // Add your test here.
    const tx = await program.methods
    .addPayer(payerAkeypair.publicKey)
    .accountsPartial({
      author: provider.publicKey,
      bill: billPda,
    })
    .rpc();
    console.log(tx);
  });

  it("Pay bill", async () => {
    // Add your test here.
    const tx = await program.methods
    .payBill()
    .accountsPartial({
      payer: payerAkeypair.publicKey,
      author: provider.publicKey,
      bill: billPda,
      solAccount: provider.publicKey,
      payerTokenAccount: null,
      authorTokenAccount: null,      
    })
    .signers([payerAkeypair])
    .rpc();
    console.log(tx);
  });

});

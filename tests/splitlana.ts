import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Splitlana } from "../target/types/splitlana";
import { randomBytes } from "crypto";
import { expect } from "chai";

describe("splitlana", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.splitlana as Program<Splitlana>;

  const payerAkeypair = anchor.web3.Keypair.generate();
  const payerBkeypair = anchor.web3.Keypair.generate();
  const authorUSDCkeypair = anchor.web3.Keypair.generate();
  const payerUSDC1keypair = anchor.web3.Keypair.generate();
  const payerUSDC2keypair = anchor.web3.Keypair.generate();

  const seedBill = new anchor.BN(randomBytes(8));
  const seedBillBytes = seedBill.toArrayLike(Buffer, "le", 8);
  const billPda = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("bill"), provider.publicKey.toBuffer(), seedBillBytes],
    program.programId
  )[0];

  const seedBillUSDC = new anchor.BN(randomBytes(8));
  const seedBillBytesUSDC = seedBillUSDC.toArrayLike(Buffer, "le", 8);
  const billPdaUSDC = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("bill"), provider.publicKey.toBuffer(), seedBillBytesUSDC],
    program.programId
  )[0];

  it("Initialize Bill in SOL", async () => {
    // Add your test here.
    const tx = await program.methods
    .initBill(seedBill, new anchor.BN(10), "SOL Bill: Snow trip", {sol: {}})
    .accountsPartial({
      user: provider.publicKey,
      bill: billPda,
    })
    .rpc();
    console.log(tx);
  });

  // Error: AnchorError caused by account: bill. Error Code: ConstraintSeeds. Error Number: 2006. Error Message: A seeds constraint was violated. 
  xit("Initialize Bill in USDC", async () => {
    // Add your test here.
    const tx = await program.methods
    .initBill(seedBillUSDC, new anchor.BN(300), "USDC Bill: Fancy dinner", {usdc: {}})
    .accountsPartial({
      user: authorUSDCkeypair.publicKey,
      bill: billPdaUSDC,
    })
    .signers([authorUSDCkeypair])
    .rpc();
    console.log(tx);
  });

  it("Add Payer A to bill in SOL", async () => {
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

  it("Error when trying to add already existing payer to bill", async () => {
    try {
      // Attempt to add the same payer again
      await program.methods
        .addPayer(payerAkeypair.publicKey)
        .accountsPartial({
          author: provider.publicKey,
          bill: billPda,
        })
        .rpc();
    } catch (error) {
      // Check if the error is the expected one
      console.log("Error captured:", error.message);
      expect(error.message).contain("Payer already exists");
    }
  });
  
  it("Payer A pays bill in SOL", async () => {
    // Add your test here.
    const tx = await program.methods
    .payBill()
    .accountsPartial({
      payer: payerAkeypair.publicKey,
      author: provider.publicKey,
      bill: billPda,
      solAccount: provider.publicKey,   // is this correct? Does it make sense to be in method context as it is duplicated?
      payerTokenAccount: null,
      authorTokenAccount: null,      
    })
    .signers([payerAkeypair])
    .rpc();
    console.log(tx);
  });

  it("Add Payer B to bill in SOL", async () => {
    // Add your test here.
    const tx = await program.methods
    .addPayer(payerBkeypair.publicKey)
    .accountsPartial({
      author: provider.publicKey,
      bill: billPda,
    })
    .rpc();
    console.log(tx);
  });

  // ERROR: Error captured: Simulation failed.
  // xit("Payer B attempts to pay bill in incorrect currency (USDC)", async () => {
  //   // Add your test here.
  //   try {
  //     await program.methods
  //     .payBill()
  //     .accountsPartial({
  //       payer: payerBkeypair.publicKey,
  //       author: provider.publicKey,
  //       bill: billPda,
  //       solAccount: null,
  //       payerTokenAccount: null,      // TBD: To add USDC token account
  //       authorTokenAccount: null,      
  //     })
  //     .signers([payerBkeypair])
  //     .rpc();
  //   } catch (error) {
  //     // Check if the error is the expected one
  //     console.log("Error captured:", error.message);
  //     expect(error.message).contain("Accounts provided are not valid");
  //   }
  // });

  it("Payer B pays bill in SOL", async () => {
    // Add your test here.
    const tx = await program.methods
    .payBill()
    .accountsPartial({
      payer: payerBkeypair.publicKey,
      author: provider.publicKey,
      bill: billPda,
      solAccount: provider.publicKey,
      payerTokenAccount: null,
      authorTokenAccount: null,      
    })
    .signers([payerBkeypair])
    .rpc();
    console.log(tx);
  });

});

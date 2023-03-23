import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { expect } from "chai";
import { HelloAnchor } from "../target/types/hello_anchor";

describe("hello_anchor", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.HelloAnchor as Program<HelloAnchor>;

  const counter = new anchor.web3.Keypair();

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize()
      .accounts({ counter: counter.publicKey })
      .signers([counter])
      .rpc();
    console.log("Your transaction signature", tx);

    const account = await program.account.counterData.fetch(counter.publicKey);
    console.log(`${account.count} is what read from the data`);

    expect(account.count.toNumber() === 0, "The counter is init with 0 count");
  });
});

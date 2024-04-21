import * as anchor from "@coral-xyz/anchor";
import { Sol404 } from "../target/types/sol404";
import { assert } from 'chai';
import { Connection, PublicKey, Keypair, LAMPORTS_PER_SOL } from '@solana/web3.js';
import { Program, AnchorProvider, web3, utils } from '@project-serum/anchor';

describe("sol404", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());


  const program = anchor.workspace.Sol404 as Program<Sol404>;

  it("Is create mint!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});

import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import * as anchor from "@project-serum/anchor";
import { program } from "./coder";
import { Program } from "@project-serum/anchor";
import { ZkTokenSim } from "../target/types/zk_token_sim";

describe("zk-token-sim", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());
  const programWorkspace = program(); //anchor.workspace.ZkTokenSim as Program<ZkTokenSim>;
  // const programWorkspace = anchor.workspace.ZkTokenSim as Program<ZkTokenSim>;

  it("Is initialized!", async () => {
    // Add your test here.
    const publicKeyTest: PublicKey = Keypair.generate().publicKey;
    const point2: PublicKey = Keypair.generate().publicKey;

    const pointTest = Buffer.from([
      0x58, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66,
      0x66, 0x66, 0x66,
    ]);

    const mint = Keypair.generate();

    const tx = await programWorkspace.rpc.initializeMint(pointTest, pointTest, {
      // accounts: {
      //   mint: mint.publicKey,
      //   authority: programWorkspace.provider.wallet.publicKey,
      //   systemProgram: SystemProgram.programId,
      //   rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      // },
      // signers: [mint],
    });
    console.log("Your transaction signature", tx);
  });
});

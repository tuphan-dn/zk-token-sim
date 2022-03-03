import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { ZkTokenSim } from "../target/types/zk_token_sim";

describe("zk-token-sim", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.ZkTokenSim as Program<ZkTokenSim>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});

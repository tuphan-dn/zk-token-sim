import { IDL } from "./../../target/types/zk_token_sim";
import {
  Coder,
  Idl,
  Program,
  Provider,
  setProvider,
} from "@project-serum/anchor";

import { SkTokenInstructionCoder } from "./instruction";
import { ZkTokenStateCoder } from "./state";
import { SkTokenAccountsCoder } from "./accounts";
import { SkTokenEventsCoder } from "./events";
import { ZkTokenSim } from "../../target/types/zk_token_sim";

/**
 * Coder for the SPL token program.
 */
export class SkTokenCoder implements Coder {
  readonly instruction: SkTokenInstructionCoder;
  readonly accounts: SkTokenAccountsCoder;
  readonly state: ZkTokenStateCoder;
  readonly events: SkTokenEventsCoder;

  constructor(idl: Idl) {
    this.instruction = new SkTokenInstructionCoder(idl);
    this.accounts = new SkTokenAccountsCoder(idl);
    this.events = new SkTokenEventsCoder(idl);
    this.state = new ZkTokenStateCoder(idl);
  }
}

export function program(): Program<ZkTokenSim> {
  return new Program<ZkTokenSim>(
    IDL,
    "nynPTAntJVuD4cZYmG1sAPu6voznhRKN9yeA7ZKfnEy",
    Provider.env(),
    coder()
  );
}

export function coder(): SkTokenCoder {
  return new SkTokenCoder(IDL);
}

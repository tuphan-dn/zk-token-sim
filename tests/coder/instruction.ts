import { Idl, InstructionCoder } from "@project-serum/anchor";
import { IdlType } from "@project-serum/anchor/dist/cjs/idl";
import { SIGHASH_GLOBAL_NAMESPACE } from "@project-serum/anchor/dist/cjs/coder/borsh/instruction";
import * as BufferLayout from "buffer-layout";
import camelCase from "camelcase";
import { snakeCase } from "snake-case";
import { sha256 } from "js-sha256";

import { pointKey, publicKey } from "./buffer-layout";

const INSTRUCTION_CODE: Record<string, number> = {};

export class SkTokenInstructionCoder implements InstructionCoder {
  private idl: Idl;
  constructor(idl: Idl) {
    this.idl = idl;
  }

  encode(ixName: string, ix: any): Buffer {
    switch (camelCase(ixName)) {
      default: {
        return this.encodeDataDefault(ixName, ix);
      }
    }
  }

  private getLayoutForEncode(ixName: string): any {
    const instIdl = this.idl.instructions.find((e) => e.name === ixName);
    if (!instIdl) throw new Error(`Invalid instruction: ${ixName}`);

    const layout = BufferLayout.union(BufferLayout.u8("instruction"));
    layout.addVariant(INSTRUCTION_CODE[ixName] || 0, BufferLayout.struct(instIdl.args.map((agr) => getFieldLayout(agr.name, agr.type))), ixName);
    return layout;
  }

  private encodeDataDefault(ixName: string, ix: any): Buffer {
    const layout = this.getLayoutForEncode(ixName);
    const instructionMaxSpan = Math.max(
      // @ts-ignore
      ...Object.values(layout.registry).map((r) => r.span)
    );

    let bff = Buffer.alloc(instructionMaxSpan);
    let span = this.getLayoutForEncode(ixName).encode({ [ixName]: ix }, bff);

    if (INSTRUCTION_CODE[ixName] !== undefined) return bff.slice(0, span);
    return Buffer.concat([sighash(SIGHASH_GLOBAL_NAMESPACE, ixName), bff.slice(1, span)]);
  }

  encodeState(_ixName: string, _ix: any): Buffer {
    throw new Error("Swap does not have state");
  }
}

function getFieldLayout(name: string, type: IdlType) {
  switch (type) {
    case "u64":
      return BufferLayout.nu64(name);
    case "publicKey":
      return publicKey(name);
    default:
      const definedType = type["defined"];
      if (definedType === "Point") return pointKey(name);

      throw new Error(`Invalid type of ${name}: ${type}`);
  }
}

function sighash(nameSpace: string, ixName: string): Buffer {
  let name = snakeCase(ixName);
  let preimage = `${nameSpace}:${name}`;
  return Buffer.from(sha256.digest(preimage)).slice(0, 8);
}

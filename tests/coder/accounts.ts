import { AccountsCoder, ACCOUNT_DISCRIMINATOR_SIZE, Idl } from "@project-serum/anchor";
import { IdlTypeDef } from "@project-serum/anchor/dist/cjs/idl";
import * as BufferLayout from "buffer-layout";

import { publicKey, pointKey } from "./buffer-layout";
import { accountSize } from "./common";

const PUBLICKEY_SIZE = 32;
const POINT_SIZE = 32;
const MINT_DATA_SIZE = PUBLICKEY_SIZE + POINT_SIZE + POINT_SIZE;

export class SkTokenAccountsCoder<A extends string = string> implements AccountsCoder {
  constructor(private idl: Idl) {}

  public async encode<T = any>(accountName: A, account: T): Promise<Buffer> {
    switch (accountName) {
      case "mint": {
        const buffer = Buffer.alloc(MINT_DATA_SIZE);
        const len = POOL_ACCOUNT_LAYOUT.encode(account, buffer);
        return buffer.slice(0, len);
      }
      default: {
        throw new Error(`Invalid account name: ${accountName}`);
      }
    }
  }

  public decode<T = any>(accountName: A, ix: Buffer): T {
    return this.decodeUnchecked(accountName, ix);
  }

  public decodeUnchecked<T = any>(accountName: A, ix: Buffer): T {
    switch (accountName) {
      case "mint": {
        return decodeMintAccount(ix);
      }
      default: {
        throw new Error(`Invalid account name: ${accountName}`);
      }
    }
  }

  // TODO: this won't use the appendData.
  public memcmp(accountName: A, _appendData?: Buffer): any {
    switch (accountName) {
      case "mint": {
        return {
          dataSize: MINT_DATA_SIZE,
        };
      }
      default: {
        throw new Error(`Invalid account name: ${accountName}`);
      }
    }
  }

  public size(idlAccount: IdlTypeDef): number {
    return ACCOUNT_DISCRIMINATOR_SIZE + accountSize(this.idl, idlAccount) ?? 0;
  }
}

function decodeMintAccount<T = any>(ix: Buffer): T {
  return POOL_ACCOUNT_LAYOUT.decode(ix) as T;
}

const POOL_ACCOUNT_LAYOUT = BufferLayout.struct([publicKey("authority"), pointKey("supplyCommitment"), pointKey("supplyDecryptionHandle")]);

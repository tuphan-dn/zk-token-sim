import { Idl, IdlError } from "@project-serum/anchor";
import { IdlEnumVariant, IdlField, IdlType, IdlTypeDef } from "@project-serum/anchor/dist/cjs/idl";

export function accountSize(idl: Idl, idlAccount: IdlTypeDef): number {
  if (idlAccount.type.kind === "enum") {
    let variantSizes = idlAccount.type.variants.map((variant: IdlEnumVariant) => {
      if (variant.fields === undefined) {
        return 0;
      }
      return variant.fields
        .map((f: IdlField | IdlType) => {
          if (!(typeof f === "object" && "name" in f)) {
            throw new Error("Tuple enum variants not yet implemented.");
          }
          return typeSize(idl, f.type);
        })
        .reduce((a: number, b: number) => a + b);
    });
    return Math.max(...variantSizes) + 1;
  }
  if (idlAccount.type.fields === undefined) {
    return 0;
  }
  return idlAccount.type.fields.map((f) => typeSize(idl, f.type)).reduce((a, b) => a + b, 0);
}

// Returns the size of the type in bytes. For variable length types, just return
// 1. Users should override this value in such cases.
function typeSize(idl: Idl, ty: IdlType): number {
  switch (ty) {
    case "bool":
      return 1;
    case "u8":
      return 1;
    case "i8":
      return 1;
    case "i16":
      return 2;
    case "u16":
      return 2;
    case "u32":
      return 4;
    case "i32":
      return 4;
    case "u64":
      return 8;
    case "i64":
      return 8;
    case "u128":
      return 16;
    case "i128":
      return 16;
    case "bytes":
      return 1;
    case "string":
      return 1;
    case "publicKey":
      return 32;
    default:
      //@ts-ignore
      if ("vec" in ty) {
        return 1;
      }
      //@ts-ignore
      if ("option" in ty) {
        return 1 + typeSize(idl, ty.option);
      }
      //@ts-ignore
      if ("coption" in ty) {
        return 4 + typeSize(idl, ty.coption);
      }
      //@ts-ignore
      if ("defined" in ty) {
        if (ty.defined === "Point") return 32;

        const filtered = idl.types?.filter((t) => t.name === ty.defined) ?? [];
        if (filtered.length !== 1) {
          console.log("ty.defined", ty.defined);
          throw new IdlError(`Type not found: ${JSON.stringify(ty)}`);
        }
        let typeDef = filtered[0];

        return accountSize(idl, typeDef);
      }
      //@ts-ignore
      if ("array" in ty) {
        let arrayTy = ty.array[0];
        let arraySize = ty.array[1];
        return typeSize(idl, arrayTy) * arraySize;
      }
      throw new Error(`Invalid type ${JSON.stringify(ty)}`);
  }
}
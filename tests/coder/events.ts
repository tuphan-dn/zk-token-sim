import { Event, EventCoder } from "@project-serum/anchor";
import { IdlEvent } from "@project-serum/anchor/dist/cjs/idl";
import { Idl } from "@project-serum/anchor";

export class SkTokenEventsCoder implements EventCoder {
  constructor(_idl: Idl) {}

  decode<E extends IdlEvent = IdlEvent, T = Record<string, string>>(_log: string): Event<E, T> | null {
    throw new Error("Swap program does not have events");
  }
}

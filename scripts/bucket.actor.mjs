import fetch from "node-fetch";
import { idlFactory } from "../src/declarations/rust_demo_backend/rust_demo_backend.did.mjs";
import { createActor } from "./actor.mjs";

const MAINNET = false;

// Production: not deploy
// local rrkah-fqaaa-aaaaa-aaaaq-cai
export const canisterId = MAINNET
  ? "UNKNOW_CANISTER_ID"
  : "s24we-diaaa-aaaaa-aaaka-cai";

export const bucketActor = createActor({
  canisterId,
  options: {
    agentOptions: {
      fetch,
      host: MAINNET ? "https://ic0.app" : "http://localhost:8000",
    },
  },
  factory: idlFactory,
});

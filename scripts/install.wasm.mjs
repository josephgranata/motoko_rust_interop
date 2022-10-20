#!/usr/bin/env node

import {IDL} from '@dfinity/candid';
import {AnonymousIdentity} from "@dfinity/agent";
import { readFile } from "fs/promises";
import {canisterId, managerActor} from "./actor.mjs";

const loadWasm = async (type) => {
  const buffer = await readFile(
    `${process.cwd()}/.dfx/local/canisters/${type}/${type}.wasm`
  );
  return [...new Uint8Array(buffer)];
};

const resetWasm = async () => {
  await managerActor.storateResetWasm();
}

const installWasm = async (wasmModule) => {
  console.log(`Installing wasm code in: ${canisterId}`);

  const chunkSize = 700000;

  const promises = [];

  const upload = async (chunks) => {
    const result = await managerActor.storageLoadWasm(chunks);
    console.log("Chunks:", result);
  };

  for (let start = 0; start < wasmModule.length; start += chunkSize) {
    const chunks = wasmModule.slice(start, start + chunkSize);
    promises.push(upload(chunks));
  }

  await Promise.all(promises);

  console.log(`Done: ${canisterId}`);
};

// TODO: just noticed that I am lucky enough the wasm never been to big
const upgradeWasm = async (wasmModule) => {
  console.log(`Upgrading wasm code.`);

  const arg = IDL.encode([IDL.Principal], [new AnonymousIdentity().getPrincipal()]);
  const result = await managerActor.installCode(arg, wasmModule);

  console.log(`Upgraded: ${result.toText()}`);
};

(async () => {
  const wasmModule = await loadWasm("rust_demo_backend");

  // Install wasm in manager
  await resetWasm();
  await installWasm(wasmModule);

  // Reinstall code
  await upgradeWasm(wasmModule);
})();

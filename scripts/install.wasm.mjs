#!/usr/bin/env node

import { readFile } from "fs/promises";
import {canisterId, managerActor} from "./actor.mjs";

const loadWasm = async (type) => {
  const buffer = await readFile(
    `${process.cwd()}/.dfx/local/canisters/${type}/${type}.wasm`
  );
  return [...new Uint8Array(buffer)];
};

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

(async () => {
  const wasmModule = await loadWasm("rust_demo_backend");
  await installWasm(wasmModule);
})();

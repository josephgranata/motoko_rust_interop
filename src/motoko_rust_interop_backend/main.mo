import Cycles "mo:base/ExperimentalCycles";
import Principal "mo:base/Principal";
import Error "mo:base/Error";
import Blob "mo:base/Blob";
import Buffer "mo:base/Buffer";
import Array "mo:base/Array";
import Prelude "mo:base/Prelude";

import IC "./ic.types";

actor Main {

  private let ic : IC.Self = actor "aaaaa-aa";

  private stable var canisterId : ?Principal = null;

  private stable var storageWasm: [Nat8] = [];

  // Source:
  // https://github.com/ORIGYN-SA/large_canister_deployer_internal
  // https://forum.dfinity.org/t/read-local-file-at-build-time-with-motoko/15945/2

  public shared ({ caller }) func storateResetWasm(): async () {
      // TODO: reject invalid caller
      storageWasm := [];
  };

  public shared ({ caller }) func storageLoadWasm(blob: [Nat8]): async ({total: Nat; chunks: Nat;}) {
      // TODO: reject invalid caller

      // Issue: https://forum.dfinity.org/t/array-to-buffer-in-motoko/15880/15
      // let buffer: Buffer.Buffer<Nat8> = Buffer.fromArray<Nat8>(storageWasm);
      // let chunks: Buffer.Buffer<Nat8> = Buffer.fromArray<Nat8>(blob);
      // buffer.append(chunks);
      // storageWasm := buffer.toArray();

      storageWasm := Array.append<Nat8>(storageWasm, blob);

      // return total wasm sizes
      return {
          total = storageWasm.size();
          chunks = blob.size();
      }
  };

  public func wasmHash(): async (Blob) {
      // return hash of the wasm
      Prelude.nyi()
  };

  public shared ({ caller }) func init() : async (Principal) {
    Cycles.add(1_000_000_000_000);

    let { canister_id } = await ic.create_canister({ settings = null });

    let self : Principal = Principal.fromActor(Main);

    let controllers : ?[Principal] = ?[canister_id, caller, self];

    // TODO: can this step be spared?
    await ic.update_settings(({
      canister_id;
      settings = {
        controllers = controllers;
        freezing_threshold = null;
        memory_allocation = null;
        compute_allocation = null;
      };
    }));

    // TODO: replace caller with effective userId
    let arg: Blob = to_candid(caller);

    await ic.install_code({
      arg;
      wasm_module = Blob.fromArray(storageWasm);
      mode = #install;
      canister_id;
    });

    return canister_id;
  };

  public shared ({ caller }) func delete() : async (Principal) {
    switch (canisterId) {
      case null {
        throw Error.reject("No bucket canisterId to delete");
      };
      case (?cId) {
        let bucket = actor (Principal.toText(cId)) : actor {
          transfer_cycles : () -> async ();
        };

        await bucket.transfer_cycles();

        await ic.stop_canister({ canister_id = cId });

        await ic.delete_canister({ canister_id = cId });

        canisterId := null;

        return cId;
      };
    };
  };

  // TODO: validate transfer cycles

  public shared ({ caller }) func installCode(canisterId : Principal, arg : Blob, wasmModule : Blob) : async () {
    await ic.install_code({
      arg = arg;
      wasm_module = wasmModule;
      mode = #upgrade;
      canister_id = canisterId;
    });
  };

};

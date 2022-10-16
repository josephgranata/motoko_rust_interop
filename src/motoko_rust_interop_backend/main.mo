import Cycles "mo:base/ExperimentalCycles";
import Principal "mo:base/Principal";
import Error "mo:base/Error";

import IC "./ic.types";

import RustDemoBackend "canister:rust_demo_backend";

actor Main {

  private let ic : IC.Self = actor "aaaaa-aa";

  private stable var canisterId : ?Principal = null;

  public shared ({ caller }) func init() : async (Principal) {
    let newCanisterId = await RustDemoBackend.create_bucket(caller);

    // Demo purpose, I keep just track of last canister I created
    canisterId := ?newCanisterId;

    return newCanisterId;
  };

  public shared ({ caller }) func delete() : async (Principal) {
    switch (canisterId) {
      case null {
        throw Error.reject("No bucket canisterId to delete");
      };
      case (?cId) {
        let deckBucket = actor (Principal.toText(cId)) : actor {
          transferCycles : () -> async ();
        };

        await deckBucket.transferCycles();

        await ic.stop_canister({ canister_id = cId });

        await ic.delete_canister({ canister_id = cId });

        canisterId := null;

        return cId;
      };
    };
  };

  public shared ({ caller }) func installCode(canisterId : Principal, arg : Blob, wasmModule : Blob) : async () {
    await ic.install_code({
      arg = arg;
      wasm_module = wasmModule;
      mode = #upgrade;
      canister_id = canisterId;
    });
  };

};

export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'delete' : IDL.Func([], [IDL.Principal], []),
    'init' : IDL.Func([], [IDL.Principal], []),
    'installCode' : IDL.Func(
        [IDL.Principal, IDL.Vec(IDL.Nat8), IDL.Vec(IDL.Nat8)],
        [],
        [],
      ),
    'storageLoadWasm' : IDL.Func(
        [IDL.Vec(IDL.Nat8)],
        [IDL.Record({ 'total' : IDL.Nat, 'chunks' : IDL.Nat })],
        [],
      ),
    'storateResetWasm' : IDL.Func([], [], []),
    'wasmHash' : IDL.Func([], [IDL.Vec(IDL.Nat8)], []),
  });
};
export const init = ({ IDL }) => { return []; };

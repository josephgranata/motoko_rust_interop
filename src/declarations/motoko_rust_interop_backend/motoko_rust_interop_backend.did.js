export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'cyclesBalance' : IDL.Func([], [IDL.Nat], ['query']),
    'delete' : IDL.Func([], [IDL.Principal], []),
    'init' : IDL.Func([], [IDL.Principal], []),
    'installCode' : IDL.Func(
        [IDL.Vec(IDL.Nat8), IDL.Vec(IDL.Nat8)],
        [IDL.Principal],
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

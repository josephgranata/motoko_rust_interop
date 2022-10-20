export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'greet' : IDL.Func([IDL.Text], [IDL.Text], ['query']),
    'transfer_cycles' : IDL.Func([], [], []),
  });
};
export const init = ({ IDL }) => { return []; };

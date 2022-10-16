export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'create_bucket' : IDL.Func([IDL.Principal], [IDL.Principal], []),
    'greet' : IDL.Func([IDL.Text], [IDL.Text], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };

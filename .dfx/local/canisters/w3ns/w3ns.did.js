export const idlFactory = ({ IDL }) => {
  const ApiKey = IDL.Record({
    'value' : IDL.Text,
    'owner' : IDL.Principal,
    'created_at' : IDL.Nat64,
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : IDL.Null });
  return IDL.Service({
    'get_all' : IDL.Func([], [IDL.Vec(ApiKey)], ['query']),
    'has_key_registered' : IDL.Func([], [IDL.Bool], ['query']),
    'name' : IDL.Func([], [IDL.Text], ['query']),
    'register_key' : IDL.Func([IDL.Text], [Result], []),
    'remove_key' : IDL.Func([], [Result], []),
    'whoami' : IDL.Func([], [IDL.Principal], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };

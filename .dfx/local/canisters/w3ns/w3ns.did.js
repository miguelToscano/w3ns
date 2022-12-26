export const idlFactory = ({ IDL }) => {
  return IDL.Service({ 'name' : IDL.Func([], [IDL.Text], ['query']) });
};
export const init = ({ IDL }) => { return []; };

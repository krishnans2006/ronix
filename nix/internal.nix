# Shared internal helpers for ronix Nix library.
{
  mkAssertion = _caller: cond: msg: assert cond || builtins.throw "ronix: ${msg}"; true;
  mkThrow = _caller: msg: builtins.throw "ronix: ${msg}";
}

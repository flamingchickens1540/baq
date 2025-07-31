{ inputs, ... }:
{
  imports = [ ./dev-shells.nix ];

  perSystem =
    { pkgs, system, ... }:
    {
      packages = {
        backend = pkgs.callPackage ./packages/backend.nix { };
        frontend = pkgs.callPackage ./packages/frontend.nix {
          inherit (inputs.bun2nix.lib.${system}) mkBunNodeModules;
        };
      };
    };
}

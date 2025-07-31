{
  perSystem =
    { pkgs, inputs', ... }:
    {
      devShells = rec {
        default = pkgs.mkShell {
          inputsFrom = [
            backend
            frontend
          ];
        };
        backend = pkgs.mkShell {
          buildInputs = with pkgs; [
            cargo
            clippy
          ];
        };
        frontend = pkgs.mkShell {
          buildInputs = with pkgs; [
            inputs'.bun2nix.packages.default
            bun
            prettier
            eslint
          ];

          shellHook = ''
            bun install --frozen-lockfile
          '';
        };
      };
    };
}

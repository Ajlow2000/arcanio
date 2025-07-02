{
    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
        cargo2nix.url = "github:cargo2nix/cargo2nix/release-0.11.0";
    };

    outputs = inputs: with inputs;
        let
           forAllSystems = nixpkgs.lib.genAttrs nixpkgs.lib.platforms.unix;
           nixpkgsFor = forAllSystems (system: import nixpkgs {
                inherit system;
                config = { };
                overlays = [ cargo2nix.overlays.default ]; # create nixpkgs that contains rustBuilder from cargo2nix overlay
            });

        in {
            packages = forAllSystems (system:
                let pkgs = nixpkgsFor."${system}"; in {
                    default = pkgs.rustPlatform.buildRustPackage {
                        pname = "arcanio";
                        version = "0.0.0";
                        src = ./.;
                        cargoHash = "sha256-zZZ3oARtlGsEQRKlItNF7y0e6fEUi9N9KCPLZBcoHh4=";
                        useFetchCargoVendor = true;
                    };
                }
            );
            devShells = forAllSystems (system:
                let pkgs = nixpkgsFor."${system}"; in {
                    default = pkgs.mkShell {
                        packages = with pkgs; [
                            rustc
                            cargo
                            rust-analyzer
                        ];
                    };
                }
            );
       };
}

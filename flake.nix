{
    inputs = {
        fenix = {
            url = "github:nix-community/fenix";
            inputs.nixpkgs.follows = "nixpkgs";
        };
        nixpkgs.url = "github:NixOS/nixpkgs";
    };

    outputs = {self, fenix, nixpkgs}:
        let pkgs = nixpkgs.legacyPackages.x86_64-linux;
            rust_complete = (fenix.outputs.packages.x86_64-linux.complete.withComponents [
                        "cargo"
                        "clippy"
                        "rustc"
                        "rustfmt"
                        "rust-src"
                    ]);
            npkgs = pkgs.nodePackages;

        in {
            defaultPackage.x86_64-linux = pkgs.hello;

            devShell.x86_64-linux =
                pkgs.mkShell {
                    buildInputs = [
                        rust_complete

                        pkgs.git

                        pkgs.httpie
                        pkgs.jq
                        pkgs.yq
                        pkgs.dig
                        pkgs.shellcheck

                        pkgs.nodejs-18_x
                        pkgs.expect
                        npkgs.typescript
                        npkgs.node-gyp
                        npkgs.serve
                    ];
                };
        };
}

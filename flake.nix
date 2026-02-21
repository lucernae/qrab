{
  description = "qrab - A Rust project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "qrab";
          version = "0.1.0";
          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = with pkgs; [ rustToolchain ];

          meta = with pkgs.lib; {
            description = "Terminal-friendly QR code generator for piped URLs";
            homepage = "https://github.com/lucernae/qrab";
            license = licenses.mit;
            maintainers = [ ];
          };
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            cargo
            rustc
            rustfmt
            clippy
          ];

          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";

          shellHook = ''
            # Create .devenv directory structure for JetBrains IDE detection
            mkdir -p .devenv/bin
            mkdir -p .devenv/lib

            # Symlink Rust toolchain binaries
            ln -sf ${rustToolchain}/bin/* .devenv/bin/

            # Symlink Rust libraries
            if [ -d "${rustToolchain}/lib" ]; then
              ln -sf ${rustToolchain}/lib/* .devenv/lib/
            fi

            echo "Development environment ready. Rust toolchain exposed in .devenv/"
          '';
        };
      }
    );
}

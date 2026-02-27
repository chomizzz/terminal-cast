{
  description = "Rust + SQLite dev shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ rust-overlay.overlays.default ];
      };
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        packages = [
          pkgs.rust-bin.nightly.latest.default
          pkgs.git
          pkgs.sqlite
          pkgs.pkg-config
          pkgs.openssl
        ];

        shellHook = ''
          export DATABASE_URL="sqlite:./terminal_manager.db"
          echo "=== Rust + SQLite dev shell (nightly) ==="
          echo "Rust: $(rustc --version)"
          echo "Cargo: $(cargo --version)"
          echo "SQLite: $(sqlite3 --version)"
          echo "DATABASE_URL=$DATABASE_URL"
        '';
      };
    };
}


{
  inputs = {
    nixpkgs.url = "nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        libraries = with pkgs;
          [ gtk3 cairo gdk-pixbuf glib dbus openssl_3 librsvg ]
          ++ lib.lists.optionals
          (system != "x86_64-darwin" && system != "aarch64-darwin")
          [ webkitgtk ]; # webkitgtk is broken on darwin currently

        packages = with pkgs;
          [
            curl
            wget
            pkg-config
            dbus
            openssl_3
            glib
            gtk3
            libsoup
            librsvg
            cargo
            rustc
            git
            clippy
            rust-analyzer
            libiconv
            nodejs_20
            nodePackages_latest.pnpm
            typeshare
            cargo-tauri
            tailwindcss-language-server
            rustfmt
          ] ++ lib.lists.optionals
          (system != "x86_64-darwin" && system != "aarch64-darwin")
          [ webkitgtk ];
      in {
        devShell = pkgs.mkShell {
          buildInputs = packages;

          shellHook = ''
            export LD_LIBRARY_PATH=${
              pkgs.lib.makeLibraryPath libraries
            }:$LD_LIBRARY_PATH
            export XDG_DATA_DIRS=${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}:$XDG_DATA_DIRS
          '';
        };
      });
}

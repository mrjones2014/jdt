{
  inputs = {
    nixpkgs.url = "nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        libraries-both = with pkgs; [
          gtk3
          cairo
          gdk-pixbuf
          glib
          dbus
          openssl_3
          librsvg
        ];

        libraries-linux = with pkgs; [ webkitgtk ];

        libraries-darwin = with pkgs; [ darwin.apple_sdk.frameworks.WebKit ];

        packages-both = with pkgs; [
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
        ];

        packages-linux = with pkgs; [
          nodejs-18_x
          nodePackages.pnpm
          pkg-config
          gtk3
          webkitgtk
          libayatana-appindicator.dev
          alsa-lib.dev
        ];

        packages-darwin = with pkgs; [
          nodejs-18_x
          nodePackages.pnpm
          curl
          wget
          pkg-config
          libiconv
          darwin.apple_sdk.frameworks.WebKit
        ];

        isLinux = system != "x86_64-darwin" && system != "aarch64-darwin";

        libraries = (if isLinux then libraries-linux else libraries-darwin)
          ++ libraries-both;
        packages = (if isLinux then packages-linux else packages-darwin)
          ++ packages-both;

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

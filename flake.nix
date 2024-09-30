{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
      in
      {
        defaultPackage = naersk-lib.buildPackage ./.;
        devShell = with pkgs; mkShell {
          buildInputs = [ 
            cargo 
            rustc 
            rustfmt 
            pre-commit 
            rustPackages.clippy

            # Added for bracket-lib support
            libiconv
            darwin.apple_sdk.frameworks.OpenGL
            darwin.apple_sdk.frameworks.ApplicationServices 
            darwin.apple_sdk.frameworks.CoreGraphics 
            darwin.apple_sdk.frameworks.CoreVideo 
            darwin.apple_sdk.frameworks.CoreFoundation 
            darwin.apple_sdk.frameworks.AppKit 
            darwin.apple_sdk.frameworks.QuartzCore 
            darwin.apple_sdk.frameworks.Foundation 
            darwin.apple_sdk.frameworks.CoreGraphics 
            darwin.apple_sdk.frameworks.CoreGraphics 
            darwin.apple_sdk.frameworks.CoreFoundation
          ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
      }
    );
}

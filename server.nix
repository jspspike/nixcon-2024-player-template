{ lib, fetchFromGitHub, rustPlatform }:

rustPlatform.buildRustPackage {
  pname = "server";
  version = "1.0.0";

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  src = ./.;
}

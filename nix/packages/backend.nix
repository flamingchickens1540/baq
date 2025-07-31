{ rustPlatform }:
rustPlatform.buildRustPackage {
  pname = "baq-backend";
  version = "0.1.0";

  src = ../../backend;
  cargoLock.lockFile = ../../backend/Cargo.lock;

  meta.mainProgram = "baq";
}

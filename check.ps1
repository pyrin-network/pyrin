cargo fmt --all
cargo clippy

$crates = @(
  "pyrin-wrpc-wasm",
  "pyrin-wallet-cli-wasm",
  "pyrin-wasm",
  "pyrin-cli",
  "pyrin-os",
  "pyrin-daemon"
)

$env:AR="llvm-ar"
foreach ($crate in $crates)
{
  Write-Output "`ncargo clippy -p $crate --target wasm32-unknown-unknown"
  cargo clippy -p $crate --target wasm32-unknown-unknown
  $status=$LASTEXITCODE
  if($status -ne 0) {
    Write-Output "`n--> wasm32 check of $crate failed`n"
    break
  }
}
$env:AR=""
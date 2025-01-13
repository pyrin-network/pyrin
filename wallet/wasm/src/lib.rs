use pyrin_cli_lib::pyrin_cli;
use wasm_bindgen::prelude::*;
use workflow_terminal::Options;
use workflow_terminal::Result;

#[wasm_bindgen]
pub async fn load_pyrin_wallet_cli() -> Result<()> {
    let options = Options { ..Options::default() };
    pyrin_cli(options, None).await?;
    Ok(())
}

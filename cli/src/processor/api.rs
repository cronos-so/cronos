use crate::errors::CliError;
use cronos_client::Client;
use solana_sdk::pubkey::Pubkey;

pub fn api_new(client: &Client, ack_authority: Pubkey, base_url: String) -> Result<(), CliError> {
    let api_pubkey =
        cronos_client::http::state::Api::pubkey(base_url.clone(), client.payer_pubkey());
    let ix =
        cronos_client::http::instruction::api_new(ack_authority, base_url, client.payer_pubkey());
    client.send_and_confirm(&[ix], &[client.payer()]).unwrap();
    println!("New api created: {}", api_pubkey);
    Ok(())
}

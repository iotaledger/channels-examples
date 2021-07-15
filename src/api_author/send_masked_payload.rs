#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use iota_streams::app_channels::api::tangle::{Address, Author, Transport};
use iota_streams::ddml::types::Bytes;

use anyhow::{bail, Result};

pub fn send_masked_payload<T: Transport>(
    author: &mut Author<T>,
    channel_address: &String,
    keyload_message_identifier: &String,
    public_payload: &String,
    masked_payload: &String,
) -> Result<Address>
where {
    // Convert the payloads to a Trytes type
    let public_payload = Bytes(public_payload.as_bytes().to_vec());
    let masked_payload = Bytes(masked_payload.as_bytes().to_vec());

    // Convert the channel address and message identifier to an Address link type
    let keyload_link = match Address::from_str(&channel_address, &keyload_message_identifier) {
        Ok(keyload_link) => keyload_link,
        Err(e) => bail!(
            "Failed to create Address from {}:{}. Reason: {}",
            &channel_address,
            &keyload_message_identifier,
            e
        ),
    };

    // Create a `SignedPacket` message and link it to the message identifier of the `Keyload` message
    // whose session key is used to encrypt the masked payload
    let (msg, seq) = author.send_signed_packet(&keyload_link, &public_payload, &masked_payload)?;

    Ok(seq.unwrap_or(msg))
}

#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use anyhow::{bail, Result};
use iota_streams::app_channels::api::tangle::{Address, Author, Transport};
use iota_streams::ddml::types::Bytes;

pub fn send_signed_message<T: Transport>(
    author: &mut Author<T>,
    channel_address: &String,
    announce_message_identifier: &String,
    public_payload: &String,
) -> Result<Address> {
    // Convert the payloads to a Trytes type
    let public_payload = Bytes(public_payload.as_bytes().to_vec());
    let empty_masked_payload = Bytes("".as_bytes().to_vec());

    // Convert the channel address and message identifier to an Address link type
    let announcement_link = match Address::from_str(&channel_address, &announce_message_identifier)
    {
        Ok(announcement_link) => announcement_link,
        Err(e) => bail!(
            "Failed to create Address from {}:{}. Reason: {}",
            &channel_address,
            &announce_message_identifier,
            e
        ),
    };

    // Create a `SignedPacket` message and link it to the message identifier of the `Announce` message
    let (msg, seq) =
        author.send_signed_packet(&announcement_link, &public_payload, &empty_masked_payload)?;

    Ok(seq.unwrap_or(msg))
}

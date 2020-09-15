#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use iota_streams::ddml::types::Bytes;
use iota_streams::app_channels::api::tangle::{Author, Address, Transport};

use anyhow::{Result, bail};

pub fn send_masked_payload<T: Transport>(author: &mut Author, channel_address: &String, keyload_message_identifier: &String, public_payload: &String, masked_payload: &String, client: &mut T, send_opt: T::SendOptions ) -> Result<Address> where T::SendOptions: Copy {

    // Convert the payloads to a Trytes type
    let public_payload = Bytes(public_payload.as_bytes().to_vec());
    let masked_payload = Bytes(masked_payload.as_bytes().to_vec());

    // Convert the channel address and message identifier to an Address link type
    let keyload_link = match Address::from_str(&channel_address, &keyload_message_identifier) {
        Ok(keyload_link) => keyload_link,
        Err(()) => bail!("Failed to create Address from {}:{}", &channel_address, &keyload_message_identifier),
    };

    // Create a `SignedPacket` message and link it to the message identifier of the `Keyload` message
    // whose session key is used to encrypt the masked payload
    let message = author.sign_packet(&keyload_link, &public_payload, &masked_payload)?;

    let mut ret_link = message.0;
    client.send_message_with_options(&ret_link, send_opt)?;
    println!("Signed message at {}", &ret_link.link.msgid);

    if message.1.is_some() {
        ret_link = message.1.unwrap();
        client.send_message_with_options(&ret_link, send_opt)?;
        println!("Sequenced message at {}", &ret_link.link.msgid);
    }
    
    println!("Published signed message");
    Ok(ret_link.link)
}
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use iota_streams::ddml::types::Bytes;
use iota_streams::app_channels::api::tangle::{Author, Address, Transport};

use failure::{Fallible, bail};

pub fn send_masked_payload<T: Transport>(author: &mut Author, channel_address: &String, keyload_message_identifier: &String, public_payload: &String, masked_payload: &String, client: &mut T, send_opt: T::SendOptions ) -> Fallible<Address> where T::SendOptions: Copy {

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
    let message = author.sign_packet(&keyload_link, &public_payload, &masked_payload).unwrap();

    // Convert the message to a bundle and send it to a node
    client.send_message_with_options(&message.0, send_opt).unwrap();
    client.send_message_with_options(&message.1.clone().unwrap(), send_opt).unwrap();
    println!("Keyload message at {}", &message.0.link.msgid);
    println!("Sequenced message at {}", &message.1.clone().unwrap().link.msgid);
    
    println!("Published private payload");
    Ok(message.1.clone().unwrap().link)
}
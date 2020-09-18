#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use iota_streams::ddml::types::Bytes;
use iota_streams::app_channels::api::tangle::{
    Author, Address, Transport
};
use anyhow::{Result, bail};

pub fn send_signed_message<T: Transport>(author: &mut Author, channel_address: &String, announce_message_identifier: &String, public_payload: &String, client: &mut T, send_opt: T::SendOptions ) -> Result<Address> where T::SendOptions: Copy {

    // Convert the payloads to a Trytes type
    let public_payload = Bytes(public_payload.as_bytes().to_vec());
    let empty_masked_payload = Bytes("".as_bytes().to_vec());

    // Convert the channel address and message identifier to an Address link type
    let announcement_link = match Address::from_str(&channel_address, &announce_message_identifier){
        Ok(announcement_link) => announcement_link,
        Err(()) => bail!("Failed to create Address from {}:{}", &channel_address, &announce_message_identifier),
    };

    // Create a `SignedPacket` message and link it to the message identifier of the `Announce` message
    let message = author.sign_packet(&announcement_link, &public_payload, &empty_masked_payload)?;

    println!("Sending signed message");

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
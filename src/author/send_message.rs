#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use iota_streams::app_channels::api::tangle::{Author, Address, Transport};
use iota_streams::protobuf3::types::Trytes;
use iota_streams::core::tbits::Tbits;
use std::str::FromStr;
use std::string::ToString;
use failure::Fallible;

pub fn send_signed_message<T: Transport>(author: &mut Author, channel_address: String, announce_message_identifier: String, public_payload: String, private_payload: String, client: &mut T, send_opt: T::SendOptions ) -> Fallible<()> {

    let public_payload = Trytes(Tbits::from_str(&public_payload).unwrap());
    let private_payload = Trytes(Tbits::from_str(&private_payload).unwrap());

    let announcement_link = Address::from_str(&channel_address, &announce_message_identifier).unwrap();

    let message = author.sign_packet(&announcement_link, &public_payload, &private_payload)?;

    // Print the information that needs to be sent to subscribers before they can read the message
    // You may want to send this identifier to the subscribers at this point
    println!("`SignedPacket` message identifier: {}", message.link.msgid.to_string());

    // Convert the message to a bundle and send it to a node
    client.send_message_with_options(&message, send_opt)?;
    println!("Sent signed message");
    Ok(())
}
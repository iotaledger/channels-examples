#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use iota_streams::app_channels::api::tangle::{Author, Address, Transport};
use iota_streams::protobuf3::types::Trytes;
use iota_streams::core::tbits::Tbits;
use std::str::FromStr;
use failure::Fallible;

pub fn send_private_payload<T: Transport>(author: &mut Author, channel_address: &String, keyload_message_identifier: &String, public_payload: &String, private_payload: &String, client: &mut T, send_opt: T::SendOptions ) -> Fallible<Address> {

    let public_payload = Trytes(Tbits::from_str(&public_payload).unwrap());

    let private_payload = Trytes(Tbits::from_str(&private_payload).unwrap());

    let keyload_link = Address::from_str(&channel_address, &keyload_message_identifier).unwrap();

    let message = author.sign_packet(&keyload_link, &public_payload, &private_payload)?;

    // Convert the message to a bundle and send it to a node
    client.send_message_with_options(&message, send_opt)?;
    println!("Published private payload");
    Ok(message.link)
}
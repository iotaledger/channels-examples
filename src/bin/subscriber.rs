#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use iota_streams::app_channels::{
    api::tangle::{Address, Transport, Subscriber}
    , message
};
use iota_lib_rs::prelude::iota_client;
use iota_streams::app::transport::tangle::client::SendTrytesOptions;
use failure::{Fallible, ensure};
use std::env;

fn get_signed_messages<T: Transport>(subscriber: &mut Subscriber, channel_address: &String, signed_message_identifier: &String, client: &mut T, recv_opt: T::RecvOptions) -> Fallible<()> {
    // Convert the channel address and message identifier to a link
    let message_link = Address::from_str(&channel_address, &signed_message_identifier).unwrap();
 
    println!("Receiving signed messages");
 
    // Use the IOTA client to find transactions with the corresponding channel address and tag
    let list = client.recv_messages_with_options(&message_link, recv_opt)?;

    // Iterate through all the transactions and stop at the first valid message
    for tx in list.iter() {
        let header = tx.parse_header()?;
        ensure!(header.check_content_type(message::signed_packet::TYPE));
        let (public_message, private_message) = subscriber.unwrap_signed_packet(header.clone())?;
        println!("Found and verified messages");
        println!("Public message: {}, private message: {}", public_message, private_message);
        break;
    }
    Ok(())
}

fn get_announcement<T: Transport>(subscriber: &mut Subscriber, channel_address: &String, announce_message_identifier: &String, client: &mut T, recv_opt: T::RecvOptions) -> Fallible<()> {
    // Convert the channel address and message identifier to a link
    let announcement_link = Address::from_str(&channel_address, &announce_message_identifier).unwrap();
 
    println!("Receiving announcement messages");
 
    // Use the IOTA client to find transactions with the corresponding channel address and tag
    let list = client.recv_messages_with_options(&announcement_link, recv_opt)?;

    // Iterate through all the transactions and stop at the first valid message
    for tx in list.iter() {
        let header = tx.parse_header()?;
        ensure!(header.check_content_type(message::announce::TYPE));
        subscriber.unwrap_announcement(header.clone())?;
        println!("Found and verified {} message", header.content_type());
        break;
    }
    Ok(())
}

fn get_keyload<T: Transport>(subscriber: &mut Subscriber, channel_address: &String, keyload_message_identifier: &String, client: &mut T, recv_opt: T::RecvOptions) -> Fallible<()> {
    // Convert the channel address and message identifier to a link
    let keyload_link = Address::from_str(&channel_address, &keyload_message_identifier).unwrap();
 
    println!("Receiving keyload messages");
 
    // Use the IOTA client to find transactions with the corresponding channel address and tag
    let list = client.recv_messages_with_options(&keyload_link, recv_opt)?;

    // Iterate through all the transactions and stop at the first valid message
    for tx in list.iter() {
        let header = tx.parse_header()?;
        ensure!(header.check_content_type(message::keyload::TYPE));
        subscriber.unwrap_keyload(header.clone())?;
        println!("Found and verified {} message", header.content_type());
        break;
    }
    Ok(())
}


fn subscribe<T: Transport>(subscriber: &mut Subscriber, channel_address: &String, announce_message_identifier: &String, client: &mut T, send_opt: T::SendOptions) -> Fallible<()> {

    // Convert the channel address and message identifier to a link
    let announcement_link = Address::from_str(&channel_address, &announce_message_identifier).unwrap();
 
    println!("Subscribing to channel");

    // Send a Subscribe message to the first valid Announce message that was found on the Tangle
    let subscription = subscriber.subscribe(&announcement_link)?;
    client.send_message_with_options(&subscription, send_opt)?;
    println!("Published `Subscribe` message");
    println!("Paste this `Subscribe` message identifier into your author's prompt  {}", subscription.link.msgid);
    Ok(())
 }

 fn main() {

    // Create a new subscriber
    // REPLACE THE SECRET WITH YOUR OWN
    let mut subscriber = Subscriber::new("MYSUBSCRIBERSECRETSTRING", true);

    // Connect to an IOTA node
    let mut client = iota_client::Client::new("https://nodes.devnet.iota.org:443");

    let args: Vec<String> = env::args().collect();

    let channel_address = &args[1];
    let announce_message_identifier = &args[2];
    let signed_message_identifier = &args[3];

    let recv_opt = ();

    match get_announcement(&mut subscriber, &channel_address.to_string(), &announce_message_identifier.to_string(), &mut client, recv_opt){
        Ok(()) => (),
        Err(error) => println!("Failed with error {}", error),
    }

    match get_signed_messages(&mut subscriber, &channel_address.to_string(), &signed_message_identifier.to_string(), &mut client, recv_opt){
        Ok(()) => (),
        Err(error) => println!("Failed with error {}", error),
    }

    // Change the default settings to use a lower minimum weight magnitude for the Devnet
    let mut send_opt = SendTrytesOptions::default();
    // default is 14
    send_opt.min_weight_magnitude = 9;
    send_opt.local_pow = false;

    match subscribe(&mut subscriber, &channel_address.to_string(), &announce_message_identifier.to_string(), &mut client, send_opt) {
        Ok(()) => (),
        Err(error) => println!("Failed with error {}", error),
    }

    let mut keyload_message_identifier = String::new();
    println!("Enter the Keyload message identifier that was published by the author:");
    std::io::stdin().read_line(&mut keyload_message_identifier).unwrap();

    match get_keyload(&mut subscriber, &channel_address.to_string(), &keyload_message_identifier.to_string(), &mut client, recv_opt){
        Ok(()) => (),
        Err(error) => println!("Failed with error {}", error),
    }

    let mut signed_private_message_identifier = String::new();
    println!("Enter the SignedPacket message identifier that was published by the author:");
    std::io::stdin().read_line(&mut signed_private_message_identifier).unwrap();

    match get_signed_messages(&mut subscriber, &channel_address.to_string(), &signed_private_message_identifier.to_string(), &mut client, recv_opt){
        Ok(()) => (),
        Err(error) => println!("Failed with error {}", error),
    }
 }

#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use iota::client as iota_client;


use iota_streams::app_channels::{
    api::tangle::{
        Address, Transport, Subscriber
    }, message
};
use iota_streams::app::transport::tangle::{
    PAYLOAD_BYTES,
    client::{
        SendTrytesOptions,
        RecvOptions
    }
};

use failure::{Fallible, ensure, bail};
use std::env;

fn get_signed_messages<T: Transport>(subscriber: &mut Subscriber, channel_address: &String, signed_message_identifier: &String, client: &mut T, recv_opt: T::RecvOptions) -> Fallible<()> 
where
T::SendOptions: Copy,
T::RecvOptions: Copy, {
        
    // Convert the channel address and message identifier to a link
    let message_link = match Address::from_str(&channel_address, &signed_message_identifier){
        Ok(message_link) => message_link,
        Err(()) => bail!("Failed to create Address from {}:{}", &channel_address, &signed_message_identifier),
    };
 
    println!("Receiving signed messages");

    let msg = client
            .recv_message_with_options(&message_link, recv_opt)
            .unwrap();
    let preparsed = msg.parse_header().unwrap();
    ensure!(
        preparsed.check_content_type(&message::SEQUENCE),
        "Wrong message type: {}",
        preparsed.header.content_type
    );
    let msg_tag = subscriber.unwrap_sequence(preparsed.clone()).unwrap();

    // Use the IOTA client to find transactions with the corresponding channel address and tag
    let list = client.recv_messages_with_options(&msg_tag, recv_opt).unwrap();
    

    // Iterate through all the transactions and stop at the first valid message
    for tx in list.iter() {
        let header = tx.parse_header().unwrap();
        ensure!(header.check_content_type(&message::SIGNED_PACKET));
        let (public_payload, masked_payload) = subscriber.unwrap_signed_packet(header.clone()).unwrap();
        println!("Found and verified messages");
        println!("Public message: {:?}, private message: {:?}", 
            String::from_utf8(public_payload.0), String::from_utf8(masked_payload.0));
        break;
    }
    Ok(())
}

fn get_announcement<T: Transport>(subscriber: &mut Subscriber, channel_address: &String, announce_message_identifier: &String, client: &mut T, recv_opt: T::RecvOptions) -> Fallible<()> 
where
    T::SendOptions: Copy,
    T::RecvOptions: Copy, {
    
    // Convert the channel address and message identifier to a link
    let announcement_link = match Address::from_str(&channel_address, &announce_message_identifier){
        Ok(announcement_link) => announcement_link,
        Err(()) => bail!("Failed to create Address from {}:{}", &channel_address, &announce_message_identifier),
    };

    println!("Receiving announcement messages");

    // Use the IOTA client to find transactions with the corresponding channel address and tag
    let list = client.recv_messages_with_options(&announcement_link, recv_opt).unwrap();
    // Iterate through all the transactions and stop at the first valid message
    for tx in list.iter() {
        let header = tx.parse_header().unwrap();
        dbg!(&header);
        ensure!(header.check_content_type(&message::ANNOUNCE));
        subscriber.unwrap_announcement(header.clone()).unwrap();
        println!("Found and verified {} message", header.content_type());
        break;
    }
    Ok(())
}

fn get_keyload<T: Transport>(subscriber: &mut Subscriber, channel_address: &String, keyload_message_identifier: &String, client: &mut T, recv_opt: T::RecvOptions) -> Fallible<()> where
T::SendOptions: Copy,
T::RecvOptions: Copy, {
    
     // Convert the channel address and message identifier to an Address link type
     let keyload_link = match Address::from_str(&channel_address, &keyload_message_identifier) {
        Ok(keyload_link) => keyload_link,
        Err(()) => bail!("Failed to create Address from {}:{}", &channel_address, &keyload_message_identifier),
    };
 
    println!("Receiving keyload messages");
    let msg = client
        .recv_message_with_options(&keyload_link, recv_opt)
        .unwrap();
    let preparsed = msg.parse_header().unwrap();
    ensure!(
        preparsed.check_content_type(&message::SEQUENCE),
        "Wrong message type: {}",
        preparsed.header.content_type
    );
    let msg_tag = subscriber.unwrap_sequence(preparsed.clone()).unwrap();

    // Use the IOTA client to find transactions with the corresponding channel address and tag
    let list = client.recv_messages_with_options(&msg_tag, recv_opt).unwrap();

    // Iterate through all the transactions and stop at the first valid message
    for tx in list.iter() {
        let header = tx.parse_header().unwrap();
        dbg!(&header);
        ensure!(header.check_content_type(&message::KEYLOAD));
        subscriber.unwrap_keyload(header.clone()).unwrap();
        println!("Found and verified {} message", header.content_type());
        break;
    }
    Ok(())
}


fn subscribe<T: Transport>(subscriber: &mut Subscriber, channel_address: &String, announce_message_identifier: &String, client: &mut T, send_opt: T::SendOptions) 
    -> Fallible<()> where
    T::SendOptions: Copy,
    T::RecvOptions: Copy, {

     // Convert the channel address and message identifier to a link
     let announcement_link = match Address::from_str(&channel_address, &announce_message_identifier){
        Ok(announcement_link) => announcement_link,
        Err(()) => bail!("Failed to create Address from {}:{}", &channel_address, &announce_message_identifier),
    };
 
    println!("Subscribing to channel");

    // Send a `Subscribe` message and link it to the message identifier 
    //of the first valid `Announce` message that was found on the Tangle
    let subscription = subscriber.subscribe(&announcement_link).unwrap();
    client.send_message_with_options(&subscription, send_opt).unwrap();
    println!("Published `Subscribe` message");
    println!("Paste this `Subscribe` message identifier into your author's command prompt  {}", subscription.link.msgid);
    Ok(())
 }

 fn main() {

    // Create a new subscriber
    // REPLACE THE SECRET WITH YOUR OWN
    let encoding = "utf-8";
    let mut subscriber = Subscriber::new("MYSUBSCRIBERSECRETSTRING", encoding, PAYLOAD_BYTES);

    // Connect to an IOTA node
    let mut client = iota_client::Client::get();
    iota_client::Client::add_node("https://nodes.devnet.iota.org:443").unwrap();

    let args: Vec<String> = env::args().collect();

    let channel_address = &args[1];
    let announce_message_identifier = &args[2];
    let signed_message_identifier = &args[3];

    let recv_opt = RecvOptions::default();

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

    if keyload_message_identifier.ends_with('\n') {
        keyload_message_identifier.pop();
    }
    if keyload_message_identifier.ends_with('\r') {
        keyload_message_identifier.pop();
    }

    match get_keyload(&mut subscriber, &channel_address.to_string(), &keyload_message_identifier.to_string(), &mut client, recv_opt){
        Ok(()) => (),
        Err(error) => println!("Failed with error {}", error),
    }

    let mut signed_private_message_identifier = String::new();
    println!("Enter the SignedPacket message identifier that was published by the author:");
    std::io::stdin().read_line(&mut signed_private_message_identifier).unwrap();

    if signed_private_message_identifier.ends_with('\n') {
        signed_private_message_identifier.pop();
    }
    if signed_private_message_identifier.ends_with('\r') {
        signed_private_message_identifier.pop();
    }

    match get_signed_messages(&mut subscriber, &channel_address.to_string(), &signed_private_message_identifier.to_string(), &mut client, recv_opt){
        Ok(()) => (),
        Err(error) => println!("Failed with error {}", error),
    }
 }

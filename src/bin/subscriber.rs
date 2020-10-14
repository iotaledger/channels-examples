#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use iota::client as iota_client;

use iota_streams::app_channels::api::tangle::{
    Address, Transport, Subscriber
};
use iota_streams::app::transport::tangle::{
    PAYLOAD_BYTES,
    client::{
        Client,
        SendTrytesOptions
    }
};

use anyhow::{Result, bail};
use std::env;

fn get_signed_messages<T: Transport>(subscriber: &mut Subscriber<T>, channel_address: &String, signed_message_identifier: &String) -> Result<()> {
        
    // Convert the channel address and message identifier to a link
    let message_link = create_message_link(&channel_address, &signed_message_identifier)?;

    let (_, public_payload, masked_payload) = subscriber.receive_signed_packet(&message_link)?;
    println!("Found and verified messages");
    println!("Public message: {:?}, private message: {:?}", 
        String::from_utf8(public_payload.0), String::from_utf8(masked_payload.0));
    Ok(())
}

fn get_announcement<T: Transport>(subscriber: &mut Subscriber<T>, channel_address: &String, announce_message_identifier: &String) -> Result<()> {
    
    // Convert the channel address and message identifier to a link
    let announcement_link = create_message_link(&channel_address, &announce_message_identifier)?;

    println!("Receiving announcement messages");
    subscriber.receive_announcement(&announcement_link)?;

    Ok(())
}

fn get_keyload<T: Transport>(subscriber: &mut Subscriber<T>, channel_address: &String, keyload_message_identifier: &String) -> Result<()> {
    
    // Convert the channel address and message identifier to an Address link type
    let keyload_link = create_message_link(&channel_address, &keyload_message_identifier)?;

    // Use the IOTA client to find transactions with the corresponding channel address and tag
    subscriber.receive_keyload(&keyload_link)?;
    Ok(())
}

fn create_message_link(address: &String, message_identifier: &String) -> Result<Address> {
    // Convert the channel address and message identifier to a link
    let message_link = match Address::from_str(&address, &message_identifier){
        Ok(message_link) => message_link,
        Err(()) => bail!("Failed to create Address from {}:{}", &address, &message_identifier),
    };

    // Use the IOTA client to find transactions with the corresponding channel address and tag
    Ok(message_link)
}

fn subscribe<T: Transport>(subscriber: &mut Subscriber<T>, channel_address: &String, announce_message_identifier: &String) -> Result<()> {

     // Convert the channel address and message identifier to a link
     let announcement_link = create_message_link(&channel_address, &announce_message_identifier)?;
 
    println!("Subscribing to channel");

    // Send a `Subscribe` message and link it to the message identifier 
    //of the first valid `Announce` message that was found on the Tangle
    let subscription = subscriber.send_subscribe(&announcement_link)?;
    println!("Published `Subscribe` message");
    println!("Paste this `Subscribe` message identifier into your author's command prompt  {}", subscription.msgid);
    Ok(())
 }

 fn main() {
    // Connect to an IOTA node

    // Change the default settings to use a lower minimum weight magnitude for the Devnet
    let mut send_opt = SendTrytesOptions::default();
    // default is 14
    send_opt.min_weight_magnitude = 9;
    send_opt.local_pow = false;

    let url = "https://nodes.devnet.iota.org:443";

    // Connect to an IOTA node
    let client: Client = Client::new(send_opt, iota_client::ClientBuilder::new().node(url).unwrap().build().unwrap());

    // Create a new subscriber
    // REPLACE THE SECRET WITH YOUR OWN
    let encoding = "utf-8";
    let mut subscriber = Subscriber::new("MYSUBSCRIBERSECRETSTRING64274572", encoding, PAYLOAD_BYTES, client);

    let args: Vec<String> = env::args().collect();

    let channel_address = &args[1];
    let announce_message_identifier = &args[2];
    let signed_message_identifier = &args[3];

    match get_announcement(&mut subscriber, &channel_address.to_string(), &announce_message_identifier.to_string()){
        Ok(()) => (),
        Err(error) => println!("Failed with error {}", error),
    }

    match get_signed_messages(&mut subscriber, &channel_address.to_string(), &signed_message_identifier.to_string()){
        Ok(()) => (),
        Err(error) => println!("Failed with error {}", error),
    }

    match subscribe(&mut subscriber, &channel_address.to_string(), &announce_message_identifier.to_string()) {
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

    match get_keyload(&mut subscriber, &channel_address.to_string(), &keyload_message_identifier.to_string()){
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

    match get_signed_messages(&mut subscriber, &channel_address.to_string(), &signed_private_message_identifier.to_string()){
        Ok(()) => (),
        Err(error) => println!("Failed with error {}", error),
    }
 }

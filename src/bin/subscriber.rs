#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use iota_streams::app_channels::{
    api::tangle::{Address, Transport, Subscriber}
    , message
};
use iota_lib_rs::prelude::iota_client;
//use iota_streams::app::transport::tangle::client::SendTrytesOptions;
use failure::{Fallible, ensure};

fn get_messages<T: Transport>(subscriber: &mut Subscriber, channel_address: String, signed_message_identifier: String, client: &mut T, recv_opt: T::RecvOptions) -> Fallible<()> {
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
        println!("Found and authenticated messages");
        println!("Public message: {}, private message: {}", public_message, private_message);
        break;
    }
    Ok(())
}

fn get_announcement<T: Transport>(subscriber: &mut Subscriber, channel_address: String, announce_message_identifier: String, client: &mut T, recv_opt: T::RecvOptions) -> Fallible<()> {
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
        println!("Found and authenticated {} message", header.content_type());
        break;
    }
    Ok(())
}

/* Currently not working as Subscribe messages are inconsistent bundles.

fn get_announcement_and_subscribe<T: Transport>(subscriber: &mut Subscriber, channel_address: String, announce_message_identifier: String, client: &mut T, send_opt: T::SendOptions, recv_opt: T::RecvOptions) -> Fallible<()> {

    // Convert the channel address and message identifier to a link
    let announcement_link = Address::from_str(&channel_address, &announce_message_identifier).unwrap();
 
    println!("Receiving announcement messages");
 
    // Use the IOTA client to find transactions with the corresponding channel address and tag
    let list = client.recv_messages_with_options(&announcement_link, recv_opt)?;

    // Iterate through all the transactions and stop at the first valid message
    let mut found_valid_msg = false;
    for tx in list.iter() {
        let header = tx.parse_header()?;
        ensure!(header.check_content_type(message::announce::TYPE));
        subscriber.unwrap_announcement(header.clone())?;
        println!("Found and authenticated {} message", header.content_type());
        found_valid_msg = true;
        break;
    }

    // Make sure that at least one of the messages were valid 
    ensure!(found_valid_msg);
    println!("Subscribing to channel");

    // Send a Subscribe message to the first valid Announce message that was found on the Tangle
    let subscription = subscriber.subscribe(&announcement_link)?;
    println!("Subscribe message identifier  {}", subscription.link.msgid);
    client.send_message_with_options(&subscription, send_opt)?;
    println!("Sent Subscribe message");
    Ok(())
 }
 */

 fn main() {

    // Create a new subscriber
    // REPLACE THE SECRET WITH YOUR OWN
    let mut subscriber = Subscriber::new("MYSUBSCRIBERSECRET", true);

    // Connect to an IOTA node
    let mut client = iota_client::Client::new("https://nodes.devnet.iota.org:443");

    // REPLACE WITH YOUR OWN CHANNEL ADDRESS AND MESSAGE IDENTIFIER
    let channel_address = "ESSPLXFXCODZEDRDZ9MEVSQAEDB9ENELCZD9YEWJZTMWFEPSONIMPATCBTKBOSEX9KCESXEWD9MIZSAPT";
    let announce_message_identifier = "RACLH9SDQZEYXOLWFG9WOLVDQHT";

    let recv_opt = ();

    match get_announcement(&mut subscriber, channel_address.to_string(), announce_message_identifier.to_string(), &mut client, recv_opt){
        Ok(()) => (),
        Err(error) => println!("Failed with error {}", error),
    }

    // REPLACE WITH YOUR OWN MESSAGE IDENTIFIER
    let signed_message_identifier = "ICOTSLXXTKVXDNWFPG9LOFUQRJS";

    match get_messages(&mut subscriber, channel_address.to_string(), signed_message_identifier.to_string(), &mut client, recv_opt){
        Ok(()) => (),
        Err(error) => println!("Failed with error {}", error),
    }

    /* Uncomment this code to subscribe to the channel
    Currently not working as Subscribe messages are inconsistent bundles.

    // Change the default settings to use a lower minimum weight magnitude for the Devnet
    let mut send_opt = SendTrytesOptions::default();
    // default is 14
    send_opt.min_weight_magnitude = 9;
    send_opt.local_pow = false;

    match get_announcement_and_subscribe(&mut subscriber, channel_address.to_string(), announce_message_identifier.to_string(), &mut client, send_opt, recv_opt) {
        Ok(()) => (),
        Err(error) => println!("Failed with error {}", error),
    }
    */
 }

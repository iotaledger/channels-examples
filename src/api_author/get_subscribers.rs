/*#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use iota_streams::app_channels::{
    api::tangle::{Address, Author, Transport}
    , message
};
use std::string::ToString;
use failure::{Fallible, ensure};


pub fn get_subscriptions_and_share_keyload<T: Transport>(author: &mut Author, client: &mut T, send_opt: T::SendOptions, recv_opt: T::RecvOptions) -> Fallible<()> {
    
    let channel_address = author.channel_address().to_string();
    
    // Authors need the message identifier to be able to get the Subscribe message
    let subscribe_message_identifier = "EILKFPZCKQFGHYHZPJL9KDLWIRZ";

    println!("Receiving Subscribe messages");

    // Use the IOTA client to find transactions with the corresponding channel address and tag
    let subscription_link = Address::from_str(&channel_address, &subscribe_message_identifier).unwrap();
    let subscribers = client.recv_messages_with_options(&subscription_link, recv_opt)?;
    
    // Iterate through all the transactions and stop at the first valid message
    let mut found_valid_msg = false;
    for tx in subscribers.iter() {
        let header = tx.parse_header()?;
        ensure!(header.check_content_type(message::subscribe::TYPE));
        // Process the message and read the subscribers' keys
        author.unwrap_subscribe(header.clone())?;
        println!("Found and authenticated {} message", header.content_type());
        found_valid_msg = true;
    }

    // Make sure that at least one of the messages were valid 
    ensure!(found_valid_msg);
    println!("Sending keyload");

    // Send a Keyload message that contains a session key for all known subscribers
    let keyload = author.share_keyload_for_everyone(&subscription_link)?;
    println!("Keyload message identifier: {}", keyload.link.msgid);
    client.send_message_with_options(&keyload, send_opt)?;
    println!("Sent Keyload message");
    Ok(())
}
*/
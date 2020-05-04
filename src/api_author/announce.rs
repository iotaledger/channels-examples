#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use iota_streams::app_channels::api::tangle::{Author, Transport};
use std::string::ToString;
use failure::Fallible;

pub fn start_a_new_channel<T: Transport>(author: &mut Author, client: &mut T, send_opt: T::SendOptions) -> Fallible<()> {

    // Create an `Announce` message to start the channel
    let announcement = author.announce()?;

    // Print the information that needs to be sent to subscribers before they can read the message
    println!("`Announce` message identifier: {}", announcement.link.msgid);

    // Convert the message to a bundle and send it to a node
    client.send_message_with_options(&announcement, send_opt)?;
    println!("Channel published");

    Ok(())
}





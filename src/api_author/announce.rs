#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use anyhow::{Result};
use iota_streams::app_channels::api::tangle::{Author, Transport, Address};

pub fn start_a_new_channel<T: Transport>(author: &mut Author, client: &mut T, send_opt: T::SendOptions) -> Result<Address> {

    // Create an `Announce` message to start the channel
    let announcement = author.announce().unwrap();

    println!("Creating a new channel");

    // Convert the message to a bundle and send it to a node
    client.send_message_with_options(&announcement, send_opt)?;
    println!("Channel published");

    let channel_address = author.channel_address().to_string();
    println!("Channel address: {}", &channel_address);

    Ok(announcement.link)
}





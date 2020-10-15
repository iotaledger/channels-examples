#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use anyhow::{Result};
use iota_streams::app_channels::api::tangle::{Author, Transport, Address};

pub fn start_a_new_channel<T: Transport>(author: &mut Author<T>) -> Result<Address> {
    // Create an `Announce` message to start the channel
    println!("Creating a new channel");
    let announce_result = author.send_announce()?;
    println!("Channel published");
    Ok(announce_result)
}
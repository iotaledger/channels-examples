#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use anyhow::{bail, Result};
use iota_streams::app_channels::api::tangle::{Address, Author, Transport};

pub fn get_subscriptions_and_share_keyload<T: Transport>(
    author: &mut Author<T>,
    channel_address: &String,
    subscribe_message_identifier: &String,
) -> Result<Address>
where
    T::SendOptions: Copy + Default,
    T::RecvOptions: Copy + Default,
{
    println!("Receiving Subscribe messages");

    // Use the IOTA client to find transactions with the corresponding channel address and tag
    let subscription_link = match Address::from_str(&channel_address, &subscribe_message_identifier)
    {
        Ok(subscription_link) => subscription_link,
        Err(e) => bail!(
            "Failed to create Address from {}:{}. Reason: {}",
            &channel_address,
            &subscribe_message_identifier,
            e
        ),
    };
    author.receive_subscribe(&subscription_link)?;

    println!("Sending keyload");

    // Publish a Keyload message for all the subscribers whose `Subscribe` messages have been processed
    let (msg, seq) = author.send_keyload_for_everyone(&subscription_link)?;
    Ok(seq.unwrap_or(msg))
}

#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use iota_streams::{
    app_channels::api::tangle::{Author}
};
use iota_lib_rs::prelude::iota_client;
use iota_streams::app::transport::tangle::client::SendTrytesOptions;
use crate::api_author::announce::start_a_new_channel;
use crate::api_author::send_message::send_signed_message;
mod api_author;

fn main() {

    //  -------- IOTA network settings ---------

    // Connect to an IOTA node
    let mut client = iota_client::Client::new("https://nodes.devnet.iota.org:443");

    // Change the default settings to use a lower minimum weight magnitude for the Devnet
    let mut send_opt = SendTrytesOptions::default();
    // default is 14
    send_opt.min_weight_magnitude = 9;
    send_opt.local_pow = false;

    // --------------- Author -------------------

    // Create a new channel
    // REPLACE THE SECRET WITH YOUR OWN
    let mut author = Author::new("MYAUTHORSECRET", 3, true);

    let channel_address = author.channel_address().to_string();
    println!("Channel address: {}", &channel_address);

    // Send the `Announce` message
    match start_a_new_channel(&mut author, &mut client, send_opt) {
        Ok(()) => (),
        Err(error) => println!("Failed with error {}", error),
    }
  
    /*
    let announce_message_identifier = "RACLH9SDQZEYXOLWFG9WOLVDQHT";

    let public_payload = "NOTICE: BREAKING CHANGES";
    let private_payload = "";

    match send_signed_message(&mut author, &channel_address, &announce_message_identifier.to_string(), &public_payload.to_string(), &private_payload.to_string(), &mut client, send_opt){
        Ok(()) => (),
        Err(error) => println!("Failed with error {}", error),
    }
    */

    /* Currently not working as Subscribe messages are inconsistent bundles.
    let recv_opt = ();
    match get_subscriptions_and_share_keyload(&mut author, &mut client, send_opt, recv_opt) {
        Ok(()) => (),
        Err(error) => println!("Failed with error {}", error),
    }
    */
}
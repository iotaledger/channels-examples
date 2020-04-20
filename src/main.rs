use iota_lib_rs::prelude::iota_client;
use iota_streams::{
    app_channels::api::tangle::{Author, Transport}
};
use iota_streams::app::transport::tangle::client::SendTrytesOptions;
use std::string::ToString;
use failure::Fallible;

fn start_a_new_channel<T: Transport>(client: &mut T, send_opt: T::SendOptions) -> Fallible<()> {
    // Create a new channel
    let mut author = Author::new("AUTHORSSECRET", 3, false);
    // Create an `Announce` message to start the channel
    let announcement_result = author.announce()?;
    // Print the information that needs to be sent to subscribers before they can read the message
    println!("Address: {}", announcement_result.link.appinst.to_string());
    println!("tag: {}", announcement_result.link.msgid.to_string());

    // Convert the message to a bundle and send it to a node
    client.send_message_with_options(&announcement_result, send_opt)?;
    println!("Announced a new channel");
    Ok(())
}

fn main() {
    // Connect to a node and pass this object to the function
    let mut client = iota_client::Client::new("https://nodes.devnet.iota.org:443");
    // Change the default settings to use a lower minimum weight magnitude for the Devnet
    let send_opt = SendTrytesOptions::default();
    // default is 14
    send_opt.min_weight_magnitude = 9; 
    match start_a_new_channel(&mut client, send_opt) {
        Ok(()) => (),
        Err(error) => println!("failed with error {}", error),
    }
}

use iota_lib_rs::prelude::iota_client;
use iota_streams::{
    app_channels::api::tangle::{Author, Transport}
};
use std::string::ToString;
use failure::Fallible;

fn start_a_new_channel<T>(client: &mut T) -> Fallible<()> where
T: Transport,
<T>::SendOptions: Copy + Default,
{
    // Create a new channel
    let mut author = Author::new("AUTHORSSECRET", 3, false);
    // Create an `Announce` message to start the channel
    let announcement_result = author.announce()?;
    // Print the information that needs to be sent to subscribers before they can read the message
    println!("Address: {}", announcement_result.link.appinst.to_string());
    println!("tag: {}", announcement_result.link.msgid.to_string());
    // Convert the message to a bundle and send it to a node
    client.send_message(&announcement_result)?;
    println!("Announced a new channel");
    Ok(())
}

fn main() {
    // Connect to a node and pass this object to the function
    let mut client = iota_client::Client::new("https://nodes.devnet.iota.org:443");
    match start_a_new_channel(&mut client) {
        Ok(()) => (),
        Err(error) => println!("failed with error {}", error),
    }
}
use iota_lib_rs::prelude::iota_client;
use iota_streams::app::transport::tangle::client::SendTrytesOptions;
use iota_streams::app_channels::{
    api::tangle::{Address, Transport, Subscriber}
};
use failure::{Fallible};

fn subscribe<T: Transport>(client: &mut T, send_opt: T::SendOptions) -> Fallible<()> {
    // Create a new subscriber
    let mut subscriber = Subscriber::new("SUBSCRIBERSSECRET", true);

    // Subscribers need the channel address and the message identifier to be able to subscribe to a channel
    let channel_address =
        "PQAGXOUVVOKQHCNPJECE9ELQOJLYIXUVHV9VAJJYVKEPMSPTABORSREYUHBMQTKRJMSNLYQVXINUVRUKO";
    let message_identifier = "JHSOMBBQVIHRWIADREESHMPWXJM";

    // Convert the channel address and message identifier to a link
    let announcement_link = Address::from_str(channel_address, message_identifier).unwrap();

    println!("Subscribing to channel");
    let subscription = subscriber.subscribe(&announcement_link)?;
    println!("  {}", subscription.link.msgid);
    client.send_message_with_options(&subscription, send_opt)?;
    Ok(())
}

fn main() {
    // Connect to a node and pass this object to the function
    let mut client = iota_client::Client::new("https://nodes.devnet.iota.org:443");
    // Change the default settings to use a lower minimum weight magnitude for the Devnet
    let mut send_opt = SendTrytesOptions::default();
    // default is 14
    send_opt.min_weight_magnitude = 9; 
    match subscribe(&mut client, send_opt) {
        Ok(()) => (),
        Err(error) => println!("failed with error {}", error),
    }
}

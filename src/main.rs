#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use iota_streams::app_channels::api::tangle::{Author, ChannelType};

mod api_author;
use crate::api_author::announce::start_a_new_channel;
use crate::api_author::get_subscribers::get_subscriptions_and_share_keyload;
use crate::api_author::send_masked_payload::send_masked_payload;
use crate::api_author::send_message::send_signed_message;

use iota_streams::app::transport::tangle::{
    client::{iota_client, Client, SendOptions},
    PAYLOAD_BYTES,
};

#[tokio::main]
async fn main() {
    //  -------- IOTA network settings ---------
    // Change the default settings to use a lower minimum weight magnitude for the Devnet
    let mut send_opt = SendOptions::default();
    send_opt.local_pow = false;

    let url = "https://chrysalis-nodes.iota.org";

    // Connect to an IOTA node
    let client: Client = Client::new(
        send_opt,
        iota_client::ClientBuilder::new()
            .with_node(url)
            .unwrap()
            .finish()
            .await
            .unwrap(),
    );

    // --------------- Author -------------------

    // Create a new channel
    // REPLACE THE SECRET WITH YOUR OWN
    let mut author = Author::new(
        "MYAUTHORSEC9ETSTRING",
        ChannelType::SingleBranch,
        client,
    );

    let channel_address = author.channel_address().unwrap().to_string();
    // announce_message is a link, thus it contains the channel address (appinst) and message identifier (msgid)
    let announce_message = start_a_new_channel(&mut author).unwrap();
    let announce_msgid = announce_message.msgid.to_string();

    let public_payload = "BREAKINGCHANGES";

    // signed_message is a link, thus it contains the channel address (appinst) and message identifier (msgid)
    let signed_message = send_signed_message(
        &mut author,
        &channel_address,
        &announce_msgid,
        &public_payload.to_string(),
    )
    .unwrap();
    println!("");
    println!("Now, in a new terminal window, use the subscriber to publish a `Subscribe` message on the channel");
    println!("");
    println!(
        "cargo +nightly run --release --bin subscriber {} {} {}",
        channel_address, announce_msgid, signed_message.msgid
    );

    let mut subscribe_message_identifier = String::new();
    println!("Enter the message identifier of the `Subscribe` message that was published by the subscriber:");
    std::io::stdin()
        .read_line(&mut subscribe_message_identifier)
        .unwrap();

    if subscribe_message_identifier.ends_with('\n') {
        subscribe_message_identifier.pop();
    }
    if subscribe_message_identifier.ends_with('\r') {
        subscribe_message_identifier.pop();
    }

    let keyload_message = get_subscriptions_and_share_keyload(
        &mut author,
        &channel_address,
        &mut subscribe_message_identifier,
    )
    .unwrap();

    println!(
        "Paste this `Keyload` message identifier in the subscriber's command prompt: {}",
        keyload_message.msgid
    );

    let masked_payload = "SUPERSECRETPAYLOAD";

    let signed_private_message = send_masked_payload(
        &mut author,
        &channel_address,
        &keyload_message.msgid.to_string(),
        &public_payload.to_string(),
        &masked_payload.to_string(),
    )
    .unwrap();

    println!(
        "Paste this `SignedPacket` message identifier in the subscriber's command prompt: {}",
        signed_private_message.msgid
    );
}

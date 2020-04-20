use iota_lib_rs::prelude::iota_client;
use iota_streams::app_channels::{
    api::tangle::{Address, Message, Transport}
};
use failure::{Fallible};

fn receive_messages<T>(client: &mut T, link: &Address) -> Fallible<Vec<Message>>
where
    T: Transport,
    // Use the default options
    <T>::RecvOptions: Copy + Default,
{
    client.recv_messages_with_options(link, T::RecvOptions::default())
}

fn main() -> Fallible<()> {
    // tangle client
    let mut client = iota_client::Client::new("https://nodes.devnet.iota.org:443");

    // Subscribers need to channel address and the message identifier to be able to find messages on a channel
    let channel_address =
        "VZGHRWHIYKQBOMWSNRFGT9VAXPZASVOPGLYHBIV9NTTAAVAVHTMOZO9XHDDRDGADHRPJWWGJJEWLWPQXY";
    let message_identifier = "UV9QBYJRVURWYGFIZENHOLUL9DD";

    // Convert the channel address and message identifier to a Tangle Address type
    let announcement_link = Address::from_str(channel_address, message_identifier).unwrap();

    println!("Receiving announcement message");
    // Use the IOTA client to find transactions with the corresponding channel address and tag
    let list = receive_messages(&mut client, &announcement_link)?;
    for tx in list.iter() {
        let header = tx.parse_header()?;
        println!("Found and authenticated {} message", header.content_type());
    }
    Ok(())
}

use iota_lib_rs::prelude::iota_client;
use iota_streams::app_channels::{
    api::tangle::{Address, Transport}
    , message
};
use failure::{Fallible, ensure};

fn get_announcement<T: Transport>(channel_address: String, announce_message_identifier: String, client: &mut T, recv_opt: T::RecvOptions) -> Fallible<()> {

   // Convert the channel address and message identifier to a link
   let announcement_link = Address::from_str(&channel_address, &announce_message_identifier).unwrap();

   println!("Receiving announcement message");

   // Use the IOTA client to find transactions with the corresponding channel address and tag
   let list = client.recv_messages_with_options(&announcement_link, recv_opt)?;
   for tx in list.iter() {
       let header = tx.parse_header()?;
       ensure!(header.check_content_type(message::announce::TYPE));
       println!("Found and authenticated {} message", header.content_type());
   }
   Ok(())
}

fn main() {

    // Connect to a node and pass this object to the function
    let mut client = iota_client::Client::new("https://nodes.devnet.iota.org:443");

    // Subscribers need the channel address and announce message identifier to be able to subscribe to a channel
    let channel_address = "PDDEPZGGFQCMGQJBZEEZOJUQLANOMLFNCVOTJQQBPABFIAPVKLMLMOFKAUWYXSCZLKPKNLR9JPTKGLXVQ";
    let announce_message_identifier = "NIVMCSOJZHMVAQQHSBTMRKKVNNX";
        
    let recv_opt = ();

    match get_announcement(channel_address.to_string(), announce_message_identifier.to_string(), &mut client, recv_opt) {
        Ok(()) => (),
        Err(error) => println!("failed with error {}", error),
    }
}

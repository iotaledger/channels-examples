# About

This repository contains sample code for you to get started with the Channels application in IOTA Streams.

You can find documentation for these examples on the [IOTA documentation portal](https://docs.iota.org/docs/channels/introduction/get-started.md).

## Getting started

To get started you need [Git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git) and [Rust](https://www.rust-lang.org/tools/install) installed on your device.

1. Clone this repository

    ```bash
    git clone https://github.com/JakeSCahill/channels-examples
    cd channels-examples
    ```

2. Open the `src/main.rs` file and change the author's secret to something secure

    ```rust
    // REPLACE THE SECRET WITH YOUR OWN
    let mut author = Author::new("MYAUTHORSECRET", 3, true);
    ```

3. Publish the channel

    ```bash
    cargo run --release --bin my_channel_app
    ```

    It may take a minute or two to download and compile the dependencies.

    In the console, you should see that the message was sent.

    ```bash
    Channel address: ESSPLXFXCODZEDRDZ9MEVSQAEDB9ENELCZD9YEWJZTMWFEPSONIMPATCBTKBOSEX9KCESXEWD9MIZSAPT
    `Announce` message identifier: RACLH9SDQZEYXOLWFG9WOLVDQHT
    Channel published
    ```

4. Copy the message identifier to the clipboard

5. Open the `src/main.rs` file, uncomment the following code, and update the variables with your message identifier and your own public payload:

    ```rust
    // REPLACE WITH YOUR OWN MESSAGE IDENTIFIER
    let announce_message_identifier = "RACLH9SDQZEYXOLWFG9WOLVDQHT";

    let public_payload = "MYPUBLICMESSAGE";
    let private_payload = "";

    match send_signed_message(&mut author, channel_address, (&announce_message_identifier).to_string(), public_payload.to_string(), private_payload.to_string(), &mut client, send_opt){
        Ok(()) => (),
        Err(error) => println!("Failed with error {}", error),
    }
    ```

    **Note:** Keep the private payload empty.

    The `private_payload` argument is encrypted only if you link the `SignedPacket` message to a `Keyload` message.

    In this case, you link the message to an `Announce` message, so the `private_payload` argument would not be encrypted anyway.

6. Comment out the following code so that you don't publish another instance of the channel

    ```rust
    /*
    // Send the `Announce` message
    match start_a_new_channel(&mut author, &mut client, send_opt) {
        Ok(()) => (),
        Err(error) => println!("Failed with error {}", error),
    }
    */
    ```

    **Note:** Authors should publish only one instance of a channel. Otherwise, subscribers will not know which channel to use.

7. Send the signed message

    ```bash
    cargo run --release --bin my_channel_app
    ```

    In the console, you should see that the message was sent.

    ```bash
    `Signed_packet` message identifier: ICOTSLXXTKVXDNWFPG9LOFUQRJS
    Sent signed message
    ```

8. Open the `bin/subscriber.rs` file, change the subscriber's secret to something secure, and update the variables with your own channel address and message identifiers

    ```rust
     // REPLACE THE SECRET WITH YOUR OWN
    let mut subscriber = Subscriber::new("MYSUBSCRIBERSECRET", true);
    ```

9. Read and verify the message

    ```bash
    cargo run --release --bin subscriber
    ```

    In the console, you should see that the subscriber was able to receive and verify the message.

    ```
    Receiving announcement messages
    Found and verified STREAMS9CHANNEL9ANNOUNCE message
    Receiving signed messages
    Found and verified messages
    Public message: MYPUBLICMESSAGE, private message: 
    ```

## Supporting the project

Channels is an alpha project that is being developed as a built-in application of the Streams framework.

If you want to support the sample code in this repository, consider:

- [Opening an issue](https://github.com/JakeSCahill/channels-examples/issues/new/choose)
- [Submitting a pull request](https://github.com/JakeSCahill/channels-examples/compare)
- [Improving the documentation](https://github.com/iotaledger/documentation/tree/develop/channels)

If you want to support the Streams project, please head over to the GitHub repository and take a look at the [contribution guidelines](https://github.com/iotaledger/streams/blob/master/.github/CONTRIBUTING.md).

## Joining the discussion

If you want to get involved in discussions about this technology, or you're looking for support, go to the #streams-discussion channel on [Discord](https://discord.iota.org/).

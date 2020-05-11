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

3. Start the author and follow the prompts

    ```bash
    cargo run --release --bin author
    ```

    When you run this command for the first time, it may take a minute or two to download and compile the dependencies.
    
    In the console, you should see something like the following:

    ```bash
    Creating a new channel
    Channel published
    Channel address: XP9QADNJGJOJTOIZDOFAOFKXQADWDKTGEMHFBNOZ9HROIPMVWPBTCUPOVKUYRKKAAFQ9XDBVSJMDALVZJ
    Sending signed message
    Published signed message

    Now, in a new terminal window, use the subscriber to publish a `Subscribe` message on the channel

    cargo run --release --bin subscriber XP9QADNJGJOJTOIZDOFAOFKXQADWDKTGEMHFBNOZ9HROIPMVWPBTCUPOVKUYRKKAAFQ9XDBVSJMDALVZJ AEFYSYRVRKKW9CPBRMXHYDNJJWK KLKGMAAQGXPHEJLQKIPKYPHEAFE

    Enter the message identifier of the `Subscribe` message that was published by the subscriber:
    ```

4. Open the `bin/subscriber.rs` file, and change the subscriber's secret to something secure

    ```rust
     // REPLACE THE SECRET WITH YOUR OWN
    let mut subscriber = Subscriber::new("MYSUBSCRIBERSECRET", true);
    ```

5. Copy the `cargo run` command and paste it into a new terminal window to start the subscriber

    ```bash
    cargo run --release --bin subscriber XP9QADNJGJOJTOIZDOFAOFKXQADWDKTGEMHFBNOZ9HROIPMVWPBTCUPOVKUYRKKAAFQ9XDBVSJMDALVZJ AEFYSYRVRKKW9CPBRMXHYDNJJWK KLKGMAAQGXPHEJLQKIPKYPHEAFE
    ```

    The first argument is the channel address, the second argument is the `Announce` message identifier, and the third argument is the `SignedPacket` message identifier.

    The subscriber needs all this information to get the messages from the channel.

6. Follow the prompts

    In the console, you should see that the subscriber was able to subscribe to the channel and read the encrypted message.

## Supporting the project

Channels is an alpha project that is being developed as a built-in protocol of the Streams framework.

If you want to support the sample code in this repository, consider:

- [Opening an issue](https://github.com/JakeSCahill/channels-examples/issues/new/choose)
- [Submitting a pull request](https://github.com/JakeSCahill/channels-examples/compare)
- [Improving the documentation](https://github.com/iotaledger/documentation/tree/develop/channels)

If you want to support the Streams project, please head over to the GitHub repository and take a look at the [contribution guidelines](https://github.com/iotaledger/streams/blob/master/.github/CONTRIBUTING.md).

## Joining the discussion

If you want to get involved in discussions about this technology, or you're looking for support, go to the #streams-discussion channel on [Discord](https://discord.iota.org/).

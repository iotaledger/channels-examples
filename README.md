# Streams samples

This repository is where we keep sample code that we use to document Streams on the [IOTA documentation portal](https://docs.iota.org).

Use this repository to try sample code and learn how to use IOTA technology with the accompanying documentation.

## Getting started

To get started you need [Git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git) and [Rust](https://www.rust-lang.org/tools/install) installed on your device.

1. Clone this repository

    ```bash 
    git clone https://github.com/JakeSCahill/channels-examples
    cd channels-examples
    ```
  
2. Announce the channel as an author

    ```bash
    cargo run --release --bin my_channel_app
    ```

    It may take a minute or two to download and compile the dependencies.

    In the console, you should see that the `Announce` message was sent.

    ```bash
    Announced a new channel
    Channel address: VZGHRWHIYKQBOMWSNRFGT9VAXPZASVOPGLYHBIV9NTTAAVAVHTMOZO9XHDDRDGADHRPJWWGJJEWLWPQXY
    Message identifier: UV9QBYJRVURWYGFIZENHOLUL9DD
    ```

3. Read and authenticate the message as a subscriber

    ```bash
    cargo run --release --bin get_announcement
    ```

  In the console, you should see that the subscriber was able to receive and authenticate the message.

    ```
    Found and authenticated Type=STREAMS9CHANNEL9ANNOUNCE message
    ```

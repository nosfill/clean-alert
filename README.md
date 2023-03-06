# clean-alert

Notification of cleaning turn via Slack API.

## Requirement

- Rust (2021 edition)
- Cargo
- Openssl
- Slack app

Related links:
1. Rust 2021: [https://www.digitalocean.com/community/tutorials/install-rust-on-ubuntu-linux](https://www.digitalocean.com/community/tutorials/install-rust-on-ubuntu-linux)
2. Cargo: [https://installati.one/install-cargo-ubuntu-20-04/](https://installati.one/install-cargo-ubuntu-20-04/)
3. Openssl” [https://idroot.us/install-openssl-ubuntu-20-04/](https://idroot.us/install-openssl-ubuntu-20-04/)
4. Slack app in the client side (to test it works)

## How to use

0. Clone this repository and move.

    ```bash
    $ git clone https://github.com/NAIST-SE/clean-alert.git
    $ cd clean-alert
    ```

1. Update `channel_info.json` (as an input file, to read members IDs list) and `member_log.csv` (as an output log file, to write members id who's in charge of that week) files.

`channel_info.json` - it contains "token" secret key for integrating bots in Slack workspace.
Generate new one from "https://api.slack.com/apps?" -> "OAuth&Permissions" -> "OAuth Tokens for Your Workspace".
Or ask the Slack workspace owner to provide the "Bot User OAuth Token". It should start with xoxb_.
Update "id" with the channel ID you like to interact with bot messages.
You can get it by right-click and copy channel link from Slack workspace. The ID is at the end of the channel link.
You may change the message, if you like to.

`member_log.csv` - write a date and two user's IDs from the member's list as a first log. You may copy the previous list or remove all logs and leave it empty. The example of two weeks run looks like this:

    ```
    2022-01-01,U0123456789,U0123456789
    2022-01-01,U0123456789,U0123456789
    ```

2. Build clean-alert

    ```bash
    $ cargo add openssl-probe
    ```

    ```bash
    $ cargo build
    ```
    
   or make build without dynamic links binary to run it elsewhere:
   
    ```bash
    $ cargo build --release
    ```

   You may clean before build, `cargo clean`
   After build cargo will generate `clean-alert` binary file, to be executed.

3. Run clean-alert

    ```bash
    $ clean-alert
    ```
> You may use docker to run cargo project: `$ docker run --rm -it -v `pwd`:/home/rust/src ekidd/rust-musl-builder cargo build --release`

    - After execution, `member_log.csv` is created.

3. If you want to use weekly , use `crontab`.

    ```bash
    # Execution every Wednesday at 15:00.
    $ 0 15 * * 3 <path to clean-alert>
    ```

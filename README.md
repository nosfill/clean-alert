# clean-alert

Notification of cleaning turn via Slack API.

## Requirement

- Rust (2021 edition)
- Cargo
- Openssl
- Slack app

## How to use

0. Clone this repository and move.

    ```bash
    $ git clone https://github.com/nosfill/clean-alert.git
    $ cd clean-alert
    ```

1. Update `channel_info.json`

`channel_info.json` contains "token" secret key for integrating bots in Slack workspace.
Generate new one from "https://api.slack.com/apps?" -> "OAuth&Permissions" -> "OAuth Tokens for Your Workspace"
or ask the Slack workspace owner to provide the "Bot User OAuth Token". It should start with xoxb_.
Update "id" with the channel ID you like to interact with bot messages.
You can get it by right-click and copy channel link from Slack workspace. The ID is at the end of the channel link.
You may change the message, if you like to.

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

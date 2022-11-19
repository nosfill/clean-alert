# clean-alert

Notification of cleaning turn via Slack API.

## Requirement

- Rust (2021 edition)
- Slack app

## How to use

0. Clone this repository and move.

    ```bash
    $ git clone https://github.com/nosfill/clean-alert.git
    $ cd clean-alert
    ```

1. Create `channel_info.json`
    
    > `channel_info.json` contains secret key. So it is not contained this repository.

    - Please refer to the following
        ```json
        {
            "token": ,   // Token owned by the Slack App.
            "id": ,      // slack app id,
            "member": ,  // member id in this workspace
            "ignore_member": // ignore member id in this workspace,
            "message_jp": "今週のゴミ掃除担当です．\n作業が終了したら，この投稿に:done:をつけてください.",
            "message_en": "You are in charge of the garbage disposal of this week.\nWhen you're done, please reaction :done: to this post."
        ```

2. Run clean-alert

    ```bash
    $ clean-alert
    ```

    - After execution, `member_log.csv` is created.

3. If you want to use for weekly , use `crontab`.

    ```bash
    # Execution every Wednesday at 15:00.
    $ 0 15 * * 3 <path to clean-alert>
    ```

## How to build

If you have docker, Please try to run this command that builds clean-alert program without linking the dynamic library.
(working directory: clean-alert/ )

```bash
$ docker run --rm -it -v `pwd`:/home/rust/src ekidd/rust-musl-builder cargo build --release
```
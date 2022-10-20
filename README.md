# safe-send-fil

FVM Actor to enable FIL transfers requiring active acceptance by the receiving party.

# Overiew

This project is primarily to test and become more familiar with the Filecoin FVM SDK.

[github.com/raulk/fil-hello-world-actor](https://github.com/raulk/fil-hello-world-actor) was the main example project used for reference in addition to the [SDK documentation](https://docs.rs/fvm_sdk/latest/fvm_sdk/sys/index.html).

Docker image from [github.com/jimpick/lotus-fvm-localnet](https://github.com/jimpick/lotus-fvm-localnet) is used for running a local FVM-enabled Filecoin testnet.

WIP: Some functions of this actor aren't working yet: see TODOs in code.

# Running

Either install [nix](https://nixos.org) and enter development environment with `nix develop` or install [rustup](https://www.rust-lang.org/tools/install), [make](https://www.gnu.org/software/make), and [just](https://github.com/casey/just). [docker](https://docs.docker.com/engine/install) is also required for running the local testnet.

Build WASM Actor for deployment

    make release

Enter interactive shell of docker container that will run testnet

    just shell

Run Filecoin node on local testnet (from new terminal)

    just run-node

Run Filecoin miner on local testnet (from new terminal)

    just run-miner

Monitor block creation on testnet (from new terminal)

    just monitor-chain

Deploy actor code to chain

    ./safe_send.sh install_actor artifacts/safe_send_fil.wasm

Instantiate an instance of actor

    ./safe_send.sh create_actor $(cat .actor_code_cid)

Create a transfer -- Method 2

    ./safe_send.sh invoke_actor \
        $(cat .actor_id) \
        2 \
        $(echo \
            "{\"hold_time\":300,\"amount\":10,\"destination\":\"abc\"}" \
            | base64 \
        )

Fund safe-send transfer

    lotus send $(cat .actor_id) 10

List transfers -- Method 4

    ./safe_send.sh invoke_actor $(cat .actor_id) 4

Accept tranfser from safe-send actor -- Method 3

    ./safe_send.sh invoke_actor $(cat .actor_id) 3

#!/usr/bin/env bash

set -xeu

# Deploy actor code to chain
./safe_send.sh install_actor artifacts/safe_send_fil.wasm

# Instantiate an instance of actor
./safe_send.sh create_actor $(cat .actor_code_cid)

# Use the actor
# |
# |
# V

AMOUNT=9

# Create a transfer
# -- Method 2
./safe_send.sh invoke_actor \
  $(cat .actor_id) \
  2 \
  $(echo \
    "{\"hold_time\":5,\"amount\":$AMOUNT,\"destination\":\"abc\"}" \
    | base64 \
  )

# List transfers (should see created transfer)
# -- Method 4
./safe_send.sh invoke_actor \
  $(cat .actor_id) \
  4

# Fund safe-send actor for transfer
lotus send $(cat .actor_id) $AMOUNT

read -p "Press enter to continue (wait for next block so actor is funded)"

# # Accept tranfser from safe-send actor
./safe_send.sh invoke_actor \
  $(cat .actor_id) \
  3

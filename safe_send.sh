#!/usr/bin/env bash

set -xeu
function install_actor() {
    local ARTIFACT="$1"
    
    local CID=$( \
        lotus chain install-actor "$ARTIFACT" \
        | sed -n 's/^Actor Code CID: //p' \
    )
    echo "$CID" > .actor_code_cid
}

function create_actor() {
    local CID="$1"

    local ID=$( \
        lotus chain create-actor $CID \
        | sed -n 's/^ID Address: //p' \
    )
    echo "$ID" > .actor_id
}

function invoke_actor() {
    local ID="$1"
    local METHOD="$2"
    local PARAMS="${3:-}"

    local RESULT=$( \
        lotus chain invoke "$ID" "$METHOD" "$PARAMS" \
        | tail -1 \
        | base64 --decode \
    )
    echo "$RESULT"
}

CMDS="\
install_actor \
create_actor \
invoke_actor \
"

CMD=$1; shift

if [[ " ${CMDS[*]} " =~ " $CMD " ]]; then
    $CMD $@
else
    echo "Invalid command '$CMD'."
    echo "Valid commands are:"
    echo $CMDS
fi

PWD := `pwd`
IMAGE := 'ghcr.io/jimpick/lotus-fvm-localnet-ready:latest'
NAME := 'lotus-fvm-localnet'

shell:
    docker run -it --rm \
        --name {{NAME}} \
        -v {{PWD}}:/wd \
        {{IMAGE}} \
        /bin/bash

run-node:
    docker exec -it {{NAME}} \
        lotus daemon \
            --lotus-make-genesis=devgen.car \
            --genesis-template=localnet.json \
            --bootstrap=false

run-miner:
    docker exec -it {{NAME}} \
        lotus-miner run \
            --nosync

monitor-chain:
    docker exec -it {{NAME}} \
        watch \
            lotus chain list \
                --count=3

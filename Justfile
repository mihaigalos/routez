@default:
    just --list --unsorted

test_udp:
    #!/bin/bash
    cargo build --release
    nc -ul 8090 >/tmp/routez.log &
    cargo run >/dev/null 2>&1 --release 127.0.0.1:7090 127.0.0.1:8090 UDP &
    sleep 3
    echo -n "HelloWorld" | nc -4u 127.0.0.1 7090 &
    pkill nc

    err() { echo -e "\e[1;31m${@}\e[0m" >&2; exit 1; }
    ok() { echo -e "\e[1;32mOK\e[0m"; }
    [ $(grep -o 'HelloWorld' /tmp/routez.log) ] && ok || err "ERROR"


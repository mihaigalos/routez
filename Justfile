@default:
    just --list --unsorted

test_tcp:
    #!/bin/bash
    cargo build --release
    nc -l 8090 >/tmp/routez_tcp.log &
    cargo run >/dev/null 2>&1 --release 127.0.0.1:7090 127.0.0.1:8090 TCP &
    sleep 3
    echo -n "HelloWorld" | nc -4 127.0.0.1 7090 &
    pkill nc

    [ $(grep -o 'HelloWorld' /tmp/routez_tcp.log) ] && just ok || just err "ERROR"

test_udp:
    #!/bin/bash
    cargo build --release
    nc -ul 8090 >/tmp/routez_udp.log &
    cargo run >/dev/null 2>&1 --release 127.0.0.1:7090 127.0.0.1:8090 UDP &
    sleep 3
    echo -n "HelloWorld" | nc -4u 127.0.0.1 7090 &
    pkill nc

    [ $(grep -o 'HelloWorld' /tmp/routez_udp.log) ] && just ok || just err "ERROR"

@err +args="ERROR":
    echo "\e[1;31m${@}\e[0m" >&2; exit 1;

@ok:
    echo "\e[1;32mOK\e[0m";

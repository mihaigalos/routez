@default:
    just --list --unsorted

@test_udp:
    cargo build --release
    nc -ul 8090 &
    cargo run >/dev/null 2>&1 --release 127.0.0.1:7090 127.0.0.1:8090 UDP &
    sleep 3
    echo -n "HelloWorld" | nc -4u 127.0.0.1 7090 &
    pkill nc

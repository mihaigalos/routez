@default:
    just --list --unsorted

tool := "routez"
docker_container_registry := "ghcr.io"
docker_user_repo := "mihaigalos"
docker_image_version := `cat Cargo.toml | grep ^version | cut -d'=' -f 2 | sed -e 's/"//g' -e 's/ //g'`
docker_image_base := docker_user_repo + "/" + tool
docker_image_dockerhub := docker_image_base + ":" + docker_image_version
docker_image_dockerhub_latest := docker_image_base + ":latest"

build:
    sudo docker build --network=host \
        -t {{ docker_image_dockerhub }} \
        -t {{ docker_image_dockerhub_latest }} \
        .

push:
    sudo docker push \
        {{ docker_image_dockerhub }} \
        {{ docker_image_dockerhub_latest }}

@test_tcp:
    cargo build --release
    nc -l 8090 >/tmp/routez_tcp.log &
    cargo run >/dev/null 2>&1 --release 127.0.0.1:7090 127.0.0.1:8090 TCP &
    sleep 3
    echo -n "HelloWorld" | nc -4 127.0.0.1 7090 &
    pkill nc

    [ $(grep -o 'HelloWorld' /tmp/routez_tcp.log) ] && just ok || just err "ERROR"

@test_udp:
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

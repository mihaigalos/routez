# routez

[![crates.io](https://img.shields.io/crates/d/routez.svg)](https://crates.io/crates/routez)

A zero-dep port forwarder. Routez supports both TCP and UDP.

![demo](docs/demo.png)

## Why?

Originally implemented to reroute traffic from Class-C IPs (`192.168.x.x`) to Docker Class-B ones (`172.18.x.x`).

Useful when creating an IPVLAN (L3) which has the Class-B IPs and when one wants to reach them from Class-C ones.

## Example usage

```bash
routez 127.0.0.1:1234 127.0.0.1:4321 TCP
```
## Example usage - config file

Create a config file with `source:port destination:port` (exactly 1 space):

```
127.0.0.1:1234 192.168.0.33:22 TCP
127.0.0.1:4321 127.0.0.1:7890 UDP
```

Now run with:
```bash
routez routez.config
```

## Docker

If you prefer running a Docker image instead of the barebones binary:

```bash
sudo docker run --rm -it --net=host --init mihaigalos/routez:latest 127.0.0.1:1234 127.0.0.1:8080 TCP
```

## Pipeview

To output color logs, have a look at [pipeview](https://github.com/mihaigalos/pipeview).

Either run:
```bash
routez 127.0.0.1:1234 127.0.0.1:4321 TCP | pipeview '(.*?) (.*?) (.*?) (.*?) -> (.*)' 'blue cyan white green bred'
```

Or create a `pipeview.toml` file in the directory you are calling `routez` from (see example in this repo) and invoke:

```bash
routez 127.0.0.1:1234 127.0.0.1:4321 UDP | pipeview
```

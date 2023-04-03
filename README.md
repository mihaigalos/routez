# routez

A zero-dep port forwarder.

![demo](docs/demo.png)

## Why?

Originally implemented to reroute traffic from Class-C IPs (`192.168.x.x`) to Docker Class-B ones (`172.18.x.x`).

Useful when creating an IPVLAN (L3) which has the Class-B IPs and when one wants to reach them from Class-C ones.

## Example usage

```bash
cargo run 127.0.0.1:1234 127.0.0.1:4321
```

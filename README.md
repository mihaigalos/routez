# routez

A zero-dep port forwarder.

![demo](docs/demo.png)

## Why?

Originally implemented to reroute traffic from Docker Class-B ones (`172.18.x.x`) to Class-C IPs (`192.168.x.x`).

Useful when creating an IPVLAN (L3) which has the Class-B IPs and when one wants to reach them from Class-C ones.

## Example usage

```bash
routez 127.0.0.1:1234 127.0.0.1:4321
```

## Pipeview

To color output logs, have a look at [pipeview](https://github.com/mihaigalos/pipeview).

Either run:
```bash
routez 127.0.0.1:1234 127.0.0.1:4321 | pipeview '(.*?) (.*?) (.*?) (.*?) -> (.*)' 'blue cyan white green bred'
```

Or create a `pipeview.toml` file in the directory you are calling `routez` from and invoke:

```bash
routez 127.0.0.1:1234 127.0.0.1:4321 | pipeview
```

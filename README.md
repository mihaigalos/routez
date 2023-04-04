# routez

A zero-dep port forwarder.

![demo](docs/demo.png)

## Why?

Originally implemented to reroute traffic from Class-C IPs (`192.168.x.x`) to Docker Class-B ones (`172.18.x.x`).

Useful when creating an IPVLAN (L3) which has the Class-B IPs and when one wants to reach them from Class-C ones.

## Example usage

```bash
routez 127.0.0.1:1234 127.0.0.1:4321
```
## Example usage - config file

Create a config file with `source:port destination:port` (exactly 1 space):

```
127.0.0.1:1234 192.168.0.33:22
127.0.0.1:4321 192.168.0.33:22
```

Now run with:
```bash
routez routez.config
```

## Pipeview

To output color logs, have a look at [pipeview](https://github.com/mihaigalos/pipeview).

Either run:
```bash
routez 127.0.0.1:1234 127.0.0.1:4321 | pipeview '(.*?) (.*?) (.*?) (.*?) -> (.*)' 'blue cyan white green bred'
```

Or create a `pipeview.toml` file in the directory you are calling `routez` from (see example in this repo) and invoke:

```bash
routez 127.0.0.1:1234 127.0.0.1:4321 | pipeview
```

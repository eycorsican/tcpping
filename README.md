# TcpPing

Simple utility to ping a TCP port.

## Example

```sh
> tcpping 1.1.1.1 53 -b en0 -i 1 -t 4
Connected to 1.1.1.1:53 in 21 ms
Connected to 1.1.1.1:53 in 3 ms
Connected to 1.1.1.1:53 in 3 ms
Connected to 1.1.1.1:53 in 7 ms
Connected to 1.1.1.1:53 in 5 ms
Connected to 1.1.1.1:53 in 5 ms
Connected to 1.1.1.1:53 in 6 ms
Connected to 1.1.1.1:53 in 2 ms
Connected to 1.1.1.1:53 in 11 ms
Connected to 1.1.1.1:53 in 9 ms
Connected to 1.1.1.1:53 in 11 ms
Connected to 1.1.1.1:53 in 3 ms
Connected to 1.1.1.1:53 in 2 ms
^C
```

## Usage

```sh
> tcpping --help
Usage: tcpping <host> <port> [-i <interval>] [-t <timeout>] [-b <boundif>]

TCP ping utility.

Options:
  -i, --interval    ping interval (Default 1)
  -t, --timeout     handshake timeout (Default 4)
  -b, --boundif     bound interface (Unix only)
  --help            display usage information
```

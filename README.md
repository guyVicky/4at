# 4at

Simple Multi-User Chat.

*This codebase has fallen into my hands and shall be used to train my Rust skills at the cost of butchering and completely trashing the original code. Sorry*

## Quick Start

### Server

```console
$ cargo run --bin server
```

Upon running the server creates `./TOKEN` where the Authentication Token is located. You will needed to connect to the Server via the Client.

### Client

```console
$ cargo run --bin client
```

In the prompt of the Client

```console
> /connect <server ip> <token>
```

# Architecture

ROS 2 nodes talk to each other through a middleware. With the default middlewares that
middleware is DDS, and DDS implementations talk to each other with a wire protocol called
RTPS. Because the wire protocol is a published standard, a node written from scratch can
join a ROS 2 system as long as it speaks the same protocol and follows the same ROS 2
conventions on top of it.

This project rebuilds every layer in Rust:

```text
 your node
    │
 prr                 client library: Node, Publisher, Subscription, Service, ...
    │
 prr-mw              middleware trait (DDS, Zenoh, in-process loopback, shared memory)
    │
 prr-dds             discovery, reliability, QoS, UDP transport
    │
 prr-rtps            RTPS messages as bytes (no sockets, no clocks)
    │
 prr-cdr             message serialization
```

Message types come from `.msg`/`.srv`/`.action` files, parsed by `prr-idl` and turned into
Rust by `prr-codegen`.

The full design, including the plan for each layer, is in `PLAN.md` at the repository root.

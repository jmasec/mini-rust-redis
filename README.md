# Mini Redis in Rust
- passion project to learn rust!
```
                       +-------------------+
                       |   kvd (server)    |  <--- binary crate
                       +---------+---------+
                                 |
      +--------------------------+-----------------------------+
      |                          |                             |
+-----v-----+             +-------v------+             +-------v------+
| kvd-core  |             | kvd-persist  |             | kvd-net      |   <-- library crates
| (engine)  |             | (persistence)|             | (network)    |
+-----------+             +--------------+             +--------------+
      |
+-----v-----+
| kvd-admin |   (metrics, config, logging)
+-----------+

And separately:
+-----------+
| kvd-cli   |   <-- binary crate (client)
+-----------+
```

```
kvd-core (lib):

The engine itself → HashMap, TTL index, collections, locking.

No networking, no persistence.

Reusable in unit tests or even in other projects.

kvd-persist (lib):

Knows how to dump/load the engine state (snapshots/AOF).

Keeps all file I/O logic separate from the in-memory engine.

kvd-net (lib):

Knows how to parse text/RESP protocol, handle TCP connections.

Doesn’t know the details of your store; just forwards commands/responses.

kvd-admin (lib):

Shared code for metrics, logging, config parsing.

Keeps “operational” stuff in one place.

kvd (bin):

The server binary (daemon).

Wires together kvd-core, kvd-persist, kvd-net, and kvd-admin.

You’ll run cargo run -p kvd to start it.

kvd-cli (bin):

A tiny client you can run in the terminal to send commands.

Makes testing super fast without needing telnet/nc.
```
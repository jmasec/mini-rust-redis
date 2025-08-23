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

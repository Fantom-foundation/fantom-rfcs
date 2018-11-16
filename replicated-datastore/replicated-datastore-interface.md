# Replicated Datastore Interface

The replicated datastore is used to keep a immutable record of all bytecode executed on the
Fantom virtual machine. A participating node on the fantom network must provide an interface
for Fantom virtual machine to retreive ordered fantom bytecode and store fantom bytecode
to the replicated data store.

This document defines the interface a replicated datastore must implement.


## Proposal 1: Key Value Store with Time Order Index

### Interface

#### Types

##### Key
Unique 256 bit key
```rust
type Key = [u8; 32];
```

##### TimeIndex
128 bit time series index where the index 0 is the oldest event
```rust
type TimeIndex =  [u8; 16];
```

##### Value
Byte array 
```rust
type Value = Vec<u8>;
```

#### Functions

##### create
Submits a value to the data replicated store for storage. This function is not guaranteed
to be successful.
```rust
fn create(k: key, v: Value);
```
##### ready_by_key
```rust
fn read_by_key(k: Key) -> Option<(TimeIndex, Value)>;
```
##### read_by_time_index

The time ordered retrieval function allows Fantom virtual machines to fetch programs
from the replicated data store and execute them in the correct order to arrive at the
same world state.

```rust
fn read_by_time_index(i: TimeIndex) -> Option<(Key, Value)>;
```

## Proposal 2

* Send transaction *Tx* of size *len* bytes into Consensus. Function returns transaction ID as a hex string.
```
fn register_transaction(Tx: &u64, len u64) -> String
```

* Check status of transaction.
```
fn check_transaction(hex_id: String) -> ConsensusStatus
```

```
enum ConsensusStatus {
    TxReceived,
    TxConsensus,
    TxUnknown,
    TxOK
}
```
*TxReceived* means transaction is known to the node but have not reached consensus level yet;
*TxConsensus* means a transaction has reached consensus level among all participants;
*TxOK* means transactions is reached consensus and consistent among all participants;
*TxUnknown* means transaction is unknown at the node.


* Register commit transaction listener
```
fn register_transaction_listener4_udp(address: Ipv4Addr, port: u16) -> bool
fn register_transaction_listener6_udp(address: Ipv6Addr, port: u16) -> bool
fn register_transaction_listener4_tcp(address: Ipv4Addr, port: u16) -> bool
fn register_transaction_listener6_tcp(address: Ipv6Addr, port: u16) -> bool
```

Once a transaction reached consensus it would be pushed to the listener as the following packet:
```
+------------------------------+
| hex id string size = A       | u16
+------------------------------+
| trnsaction hex id string     | A x u8
|                              |
+------------------------------+
| transaction payload size = B |
+------------------------------+
| transaction payload          | B x u8
|                              |
+------------------------------+
```





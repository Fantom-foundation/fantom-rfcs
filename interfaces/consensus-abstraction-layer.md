# Consensus Abstraction Layer

This document defines the interface a Fantom Consensus algorithm must provide.

## Interface

### Top level

* Send transaction *Tx* of suze *len* bytes into Consensus. Function returns transaction ID as a hex string.
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


```
fn create(Va) -> Key
```


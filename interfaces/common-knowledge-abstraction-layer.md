# Common Knowlegde Abstraction Layer

A role of the consensus Layer in the Fantom framework is to provide a common store
of knowledge for the Fantom virtual machine. A consensus participant must provide an
interface for Fantom programs to add and retrieve data from the common knowledge store.

This document defines the interface a consensus participant must provide to the Fantom
Virtual Machine..


## Proposal 1: Time Ordered Key Value Store Interface

The time ordered key value store interface allows for adding data or Fantom programs to the
common knowledge store, retrieving data by key and retrieving data by time order.

The time ordered retrieval function allows provides a convenient interface for the Fantom
virtual machine to fetch programs 

```
type Key = u256

//Fantom program bytecode
type Program = [u8]

type Data = [u8]

type Value = Program | Data 


function create(Value) -> Key;

function read_by_key(Key) -> Option<(u128, Value)>;

function read_by_time_order(u128) -> Option<(Key, Value)>;
  
```

## Proposal 2

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





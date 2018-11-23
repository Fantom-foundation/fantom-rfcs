# Fantom Framework

The Fantom framework is a framework for building distributed applications that run on a
trustless network.

The Fantom framework consists of two main components:

1. Replicated Transaction Log
2. Virtual Machine

## Definitions

##### Fantom Network
The Fantom network consists of a group of participants that agree or disagree on
the occurence of events using the Lachesis consensus protocol.

## Replicated Transaction Log

Fantom network participants propose transactions to be appended to the replicated transaction log.
Transactions are validated and appended to the transaction through the the Lachesis consensus protocol.

The replicated event log is used to keep a immutable record of all bytecode
executed on the Fantom virtual machine. A participating node on the fantom network must
provide an interface for Fantom virtual machine to retreive ordered fantom bytecode and 
store fantom bytecode from the replicated event log.

Typically a consensus algorithm would be used to implement this replicated transaction log.

### Interface

#### Types

##### Transaction Hash
Unique 256 bit key
```rust
type TxHash = [u8; 32];
```

##### Absolute Order
64 bit unsigned integer representing the absolute order of the transaction where the index 0 is 
absolute order of the oldest event.
```rust
type TimeIndex = u64;
```

##### Transaction Body
Byte array that typically contains the
```rust
type TxBody = Vec<u8>;
```

#### Functions

##### Create
Submits a transaction to the network for confirmation. This function is not guaranteed
to be successful. read_by_key can be used to check if this function call
was successful.
 
```rust
fn submit_tx(k: key, v: Value) -> Result<TimeIndex, TransactionStatus>;
```

##### Read by Key
Fetch a stored value by its key. A None return value indicates that
the there is no value corresponding to that key. This function can be 
used to whether a create operation has failed or not.
```rust
fn get_tx_by_key(k: Key) -> Option<(TimeIndex, Value)>;
```
##### Read by Time Index

The time ordered retrieval function allows Fantom virtual machines to fetch programs
from the replicated data store and execute them in the correct order to arrive at the
same world state.

```rust
fn read_by_time_index(i: TimeIndex) -> Option<(Key, Value)>;
```

## Virtual Machine


The role of the Fantom virtual machine is to execute Fantom programs in a deterministic manner
to build a world state for Fantom applications.

This document defines the interface a Fantom Virtual machine must implement.

### Fantom World State

Todo: describe the world state data structure and how fantom programs interact with it.

``` rust
type State = Vec<u8>
```

### State Transition

The Fantom virtual machine must implement a function to transition from 
the fantom world state to another. Invalid programs must return a error result.
Valid programs return the new world state.

``` rust
fn transition(State, Program) -> Result<State, Error>;
```

A program consists of virtual machine bytecode.

``` rust
type Program = Vec<u8>;
```

The Fantom virtual machine acquires programs from the replicated data store in time order 
and execute them to build the world state. The Fantom virtual machine uses the
read_by_time_index API provided by the replicated datastore to acquire programs in time order.


### Verified Computation Certificates

The Ethereum state machine requires every participant to compute the state  
An alternative is for a particular client

Unlike the Ethereum virtual machine, the Fantom virtual machine does not require every participant
to compute the next state to verify it. Participants must generate a certificate to prove that
the state transition caused by their submitted transaction is valid. Other participants only need
to verify the certificate to esnure the computed state is correct.
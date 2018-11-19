# Virtual Machine Abstraction Layer

The role of the Fantom virtual machine is to execute Fantom programs in a deterministic manner
to build a world state for Fantom applications.

This document defines the interface a Fantom Virtual machine must implement.

## Fantom World State

Todo: describe the world state data structure and how fantom programs interact with it.

``` rust
type State = Vec<u8>
```

## State Transition

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



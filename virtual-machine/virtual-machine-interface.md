# Virtual Machine Abstraction Layer

The role of the Fantom virtual machine is to execute Fantom programs in a deterministic manner.

This document defines the interface a Fantom Virtual machine must implement;

At its bare minimum, the Fantom virtual machine must implement a function to transition from 
the fantom world state to another.

```
type Program = [u8]

type State = 

fn transition(State, Program) -> Result<State>;
```
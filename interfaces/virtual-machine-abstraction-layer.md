# Virtual Machine Abstraction Layer

The role of the Fantom virtual machine is to execute Fantom programs in a deterministic manner.
Fantom programs
 

This document defines the interface a Fantom Virtual machine must provide.

## Proposal 1: Sate

The Fantom virtual machine must implement two functions:
1. Validate the Program
2. State Fr

```
type Program = Vec<u8>
function validate_program(gas: u64, program: Program) -> bool;
function transition(state: State, program: Program) -> Result<State>;
```
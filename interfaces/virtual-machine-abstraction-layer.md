# Virtual Machine Abstraction Layer

This document defines the interface a Fantom Virtual machine must provide.

## Interface

```
fn validate_program(gas: u64, program: Program) -> bool;
fn transition(state: State, program: Program) -> Result<State>;
```
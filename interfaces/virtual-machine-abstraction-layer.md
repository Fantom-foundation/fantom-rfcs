# Virtual Machine Abstraction Layer

This document defines the interface a Fantom Virtual machine must provide.

## Interface

```
fn validate_program(program: Program) -> bool;
fn transition(state: State, program: Program) -> Result<State>;
```
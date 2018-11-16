# Fantom Programs

Fantom programs are programs that run on the Fantom Virtual Machine. Fantom programs must
compile to FVM bytecode. 

## Fantom Bytecode

### Gas Costs

Fantom programs cost gas to run. Each instruction of the Fantom ISA has a cost to execute.


## Fantom API/stdlib

The Fantom standard library is a WASM API for applications that run on the Fantom Virtual
Machine. The Fantom API provides Fantom programs an interface to store data on the FVM
and communicate with Fantom programs.

##### Load Program
Loads an existing Fantom program to interact with.

##### Create Persistent Variable

### Communicating with other Fantom Programs

Each fantom transaction has a unique 256 bit identifier. A Transaction contains WASM 
or WASM/AST code.

A smart contract can have public functions. Public functions from a transaction can be
called from another transaction by referencing that transactions hash.



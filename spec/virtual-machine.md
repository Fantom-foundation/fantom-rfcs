# Fantom Virtual Machine

The fantom virtual machine is used to compute the state the of the fantom network. The fantom virtual machine applies 
fantom programs current fantom world state to computes a new fantom world state.

```
fn transition(state: State, program: Vec<u8>) -> Result<State> 
```

## Referencing Other Transactions

Each fantom transaction has a unique 256 bit identifier. A Transaction contains WASM 
or WASM/AST code.

A smart contract can have public functions. Public functions from a transaction can be
called from another transaction by referencing that transactions hash.

```
//sending bob 1000 dollars
use fvm;
use bobs_account(0x928382988...);

fn main() -> Result<()> {
    let bobs_account = fvm::load_contract("0x92848294829)?;
    bobs_account.send_money(1000)
}
```

```
use fvm;
//bobs account contract

let mut balance = fvm.disk_alloc()

```

## Gas Price

### 

 

## Fantom Standard Library

The Fantom standard library is a WASM API for applications that run on the Fantom Virtual Machine.


### Disk Storage
No disk space

###


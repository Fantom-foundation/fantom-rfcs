# Consensus Abstraction Layer

This document defines the interface a Fantom network participant must provide.

A Fantom network participant can
1. submit data to be written to the common 
2. knowledge store and read data in the common knowledge store. 


## Participant Interface


```
type Key = [u8; 4]
type Value = Vec<u8>

fn write(v: value) -> Key;
fn read(ks: Vec<Key>) -> Vec<Some<Value>>;

```


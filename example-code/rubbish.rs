
struct Transaction {

}

struct State {

}

struct Event {
    transaction: Transaction,
    self_parent: Bytes32,
    other_parent: Bytes32,

}

struct Block {
    atropos: Bytes32,
    clothos: Vec<Bytes32>,
    roots: Vec<Bytes32>,
    ordered_events: Vec<Event>,
    prev_block:
}


pub trait FantomVirtualMachine {
    fn transition(state: State, tx: Transaction) -> Result<State> {

    }
}

pub trait ConsensusParticipant {
    fn submitTransaction(tx: Transaction);
    fn getTransactionsFromBlock(block_number: u64) -> Vec<Transaction>;
    fn getLatestBlockNumber() -> u64;
}



//sending bob 1000 dollars
use fvm;
use 0x928382988::{bobs_account};

fn main() -> Result<()> {
    let bobs_account = fvm::load_program("0x92848294829")?;
    bobs_account.send_money(1000);
    Ok()
}

use fvm;
//bobs account contract

// allocates a peristent variable on the fantom virtual machine;
let mut balance: u64 = fvm::alloc(2000);

pub fn send_money(amount: u64) {
    bobs_balance += amount;
}


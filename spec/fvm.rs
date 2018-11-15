
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
}

struct FVM {

}

impl State {
    pub fn transition(&self, tx: Transaction) -> Result<State> {

    }
}

pub trait ConsensusParticipant {
    fn submitTransaction(tx: Transaction);
    fn getTransactionsFromBlock(block_number: u64) -> Vec<Transaction>;
    fn getLatestBlockNumber() -> u64;
}


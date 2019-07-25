use bytes::Bytes;
use ethereum_types::{Address, H256};

type Bytes32 = H256;
type Predicate = Address;

#[derive(Debug, Eq, PartialEq)]
pub struct Property {
    predicate: Predicate,
    input: Bytes,
}

#[derive(Debug, Eq, PartialEq)]
pub struct ImplicationProofElement {
    implication: Property,
    implicationWitness: Bytes,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Decision {
    outcome: bool,
    implicationProof: Vec<ImplicationProofElement>,
}

impl Decision {
    pub fn new(outcome: bool, implicationProof: Vec<ImplicationProofElement>) -> Self {
        Decision {
            outcome,
            implicationProof,
        }
    }
    pub fn get_outcome(&self) -> bool {
        self.outcome
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum DecisionStatus {
    Decided(bool),
    Undecided,
}

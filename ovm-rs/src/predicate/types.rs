use bytes::Bytes;
use ethereum_types::Address;

type Predicate = Address;

#[derive(Debug, Eq, PartialEq)]
pub struct Property {
    predicate: Predicate,
    input: Bytes,
}

#[derive(Debug, Eq, PartialEq)]
pub struct ImplicationProofElement {
    implication: Property,
    implication_witness: Bytes,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Decision {
    outcome: bool,
    implication_proof: Vec<ImplicationProofElement>,
}

impl Decision {
    pub fn new(outcome: bool, implication_proof: Vec<ImplicationProofElement>) -> Self {
        Decision {
            outcome,
            implication_proof,
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

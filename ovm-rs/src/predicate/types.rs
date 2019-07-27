use bytes::Bytes;
use ethereum_types::Address;

type Predicate = Address;
pub trait Input {}
pub trait Witness {}

#[derive(Debug, Eq, PartialEq)]
pub struct Property {
    predicate: Predicate,
    input: Bytes,
}

impl Property {
    pub fn new(
        predicate: Predicate,
        input: Bytes,
    ) -> Self {
        Property {
            predicate,
            input,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ImplicationProofElement {
    implication: Property,
    implication_witness: Bytes,
}

impl ImplicationProofElement {
    pub fn new(
        implication: Property,
        implication_witness: Bytes,
    ) -> Self {
        ImplicationProofElement {
            implication,
            implication_witness,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum DecisionStatus {
    Decided(bool),
    Undecided,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Decision {
    outcome: DecisionStatus,
    implication_proof: Vec<ImplicationProofElement>,
}

impl Decision {
    pub fn new(outcome: DecisionStatus, implication_proof: Vec<ImplicationProofElement>) -> Self {
        Decision {
            outcome,
            implication_proof,
        }
    }
    pub fn get_outcome(&self) -> &DecisionStatus {
        &self.outcome
    }
}

pub trait Decider {
    type Input;
    type Witness;
    fn decide(&self, input: &Self::Input, witness: Self::Witness) -> Decision;
    fn check_decision(&self, input: &Self::Input) -> Decision;
}

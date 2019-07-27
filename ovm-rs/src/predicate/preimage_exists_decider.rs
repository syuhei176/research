use super::types::{Decider, Decision, DecisionStatus, ImplicationProofElement, Property, Witness};
use crate::layer2::Layer2Core;
use crate::predicate::abi::{Decodable, Encodable};
use ethabi::{ParamType, Token};
use ethereum_types::{Address, H256};
use plasma_db::traits::kvs::{BaseDbKey, KeyValueStore};
use tiny_keccak::Keccak;
use bytes::Bytes;

pub struct Verifier {}
impl Verifier {
    pub fn new() -> Self {
        Verifier {}
    }
    pub fn hash(&self, preimage: &Bytes) -> H256 {
        Self::static_hash(preimage)
    }
    pub fn static_hash(preimage: &Bytes) -> H256 {
        let mut sha3 = Keccak::new_sha3_256();

        sha3.update(preimage.as_ref());

        let mut res: [u8; 32] = [0; 32];
        sha3.finalize(&mut res);
        H256::from(res)
    }
}

pub struct PreimageExistsInput {
    verifier: Verifier,
    parameters: Bytes,
    hash: H256,
}

impl PreimageExistsInput {
    pub fn new(verifier: Verifier, parameters: Bytes, hash: H256) -> Self {
        PreimageExistsInput {
            verifier,
            parameters,
            hash,
        }
    }
    pub fn get_parameters(&self) -> &Bytes {
        &self.parameters
    }
}

#[derive(Clone, Debug)]
pub struct PreimageExistsWitness {
    preimage: Bytes,
}

impl PreimageExistsWitness {
    pub fn new(preimage: Bytes) -> Self {
        PreimageExistsWitness { preimage }
    }
}

impl Witness for PreimageExistsWitness {}


impl Encodable for PreimageExistsWitness {
    fn to_abi(&self) -> Vec<u8> {
        ethabi::encode(&self.to_tuple())
    }
    fn to_tuple(&self) -> Vec<Token> {
        vec![Token::Bytes(self.preimage.to_vec())]
    }
}

impl Decodable for PreimageExistsWitness {
    type Ok = PreimageExistsWitness;
    fn from_tuple(tuple: &[Token]) -> Self {
        let preimage = tuple[0].clone().to_bytes();
        PreimageExistsWitness::new(Bytes::from(preimage.unwrap()))
    }
    fn from_abi(data: &[u8]) -> Option<Self> {
        ethabi::decode(&[ParamType::Bytes], data)
            .map(|decoded| Self::from_tuple(&decoded))
            .ok()
    }
}

pub struct PreimageExistsDecisionValue {
    decision: bool,
    witness: PreimageExistsWitness,
}

impl PreimageExistsDecisionValue {
    pub fn new(decision: bool, witness: PreimageExistsWitness) -> Self {
        PreimageExistsDecisionValue { decision, witness }
    }
    pub fn get_decision(&self) -> bool {
        self.decision
    }
    pub fn get_witness(&self) -> &PreimageExistsWitness {
        &self.witness
    }
}

impl Encodable for PreimageExistsDecisionValue {
    fn to_abi(&self) -> Vec<u8> {
        ethabi::encode(&self.to_tuple())
    }
    fn to_tuple(&self) -> Vec<Token> {
        vec![
            Token::Bool(self.decision),
            Token::Bytes(self.witness.to_abi()),
        ]
    }
}

impl Decodable for PreimageExistsDecisionValue {
    type Ok = PreimageExistsDecisionValue;
    fn from_tuple(tuple: &[Token]) -> Self {
        let decision = tuple[0].clone().to_bool();
        let witness = tuple[1].clone().to_bytes();
        PreimageExistsDecisionValue::new(
            decision.unwrap(),
            PreimageExistsWitness::from_abi(&witness.unwrap()).unwrap(),
        )
    }
    fn from_abi(data: &[u8]) -> Option<Self> {
        ethabi::decode(&[ParamType::Bool, ParamType::Bytes], data)
            .map(|decoded| Self::from_tuple(&decoded))
            .ok()
    }
}

pub struct PreimageExistsDecider {
    layer2: Layer2Core,
}

impl Default for PreimageExistsDecider {
    fn default() -> Self {
        PreimageExistsDecider {
            layer2: Default::default(),
        }
    }
}

impl Decider for PreimageExistsDecider {
    type Input = PreimageExistsInput;
    type Witness = PreimageExistsWitness;

    fn decide(&self, input: &PreimageExistsInput, witness: PreimageExistsWitness) -> Decision {
        let preimage = &witness.preimage;

        if input.verifier.hash(preimage) != input.hash {
            panic!("invalid preimage")
        }

        let decision_key = input.hash;
        let decision_value = PreimageExistsDecisionValue::new(true, witness.clone());
        if self
            .layer2
            .bucket("preimage_exists_decider")
            .put(
                &BaseDbKey::from(decision_key.as_bytes()),
                &decision_value.to_abi(),
            )
            .is_err()
        {
            panic!("failed to store data")
        }

        Decision::new(DecisionStatus::Decided(true), vec![])
    }
    fn check_decision(&self, input: &PreimageExistsInput) -> Decision {
        let decision_key = input.hash;
        let result = self
            .layer2
            .bucket("preimage_exists_decider")
            .get(&BaseDbKey::from(decision_key.as_bytes()));
        if let Ok(Some(decision_value_bytes)) = result {
            let decision_value =
                PreimageExistsDecisionValue::from_abi(&decision_value_bytes).unwrap();
            return Decision::new(
                DecisionStatus::Decided(decision_value.get_decision()),
                vec![ImplicationProofElement::new(
                    Property::new(Address::zero(), input.get_parameters().clone()),
                    Bytes::from(decision_value.get_witness().to_abi()),
                )],
            );
        }

        Decision::new(DecisionStatus::Undecided, vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Decision, DecisionStatus, PreimageExistsDecider, PreimageExistsInput, PreimageExistsWitness, Verifier,
    };
    use crate::predicate::types::Decider;
    use bytes::Bytes;


    #[test]
    fn test_decide() {
        let preimage_exists_decider: PreimageExistsDecider = Default::default();
        let input = PreimageExistsInput::new(
            Verifier::new(),
            Bytes::from(""),
            Verifier::static_hash(&Bytes::from("test")),
        );
        let decided: Decision =
            preimage_exists_decider.decide(&input, PreimageExistsWitness::new(Bytes::from("test")));
        assert_eq!(decided.get_outcome(), &DecisionStatus::Decided(true));
        let status = preimage_exists_decider.check_decision(&input);
        assert_eq!(status.get_outcome(), &DecisionStatus::Decided(true));
    }

}

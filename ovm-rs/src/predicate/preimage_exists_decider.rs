use super::types::{Decision, DecisionStatus};
use crate::layer2::Layer2Core;
use bytes::Bytes;
use ethabi::{ParamType, Token};
use ethereum_types::H256;
use plasma_db::traits::kvs::{BaseDbKey, Bucket, KeyValueStore};
use tiny_keccak::Keccak;

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
}

#[derive(Clone, Debug)]
pub struct PreimageExistsWitness {
    preimage: Bytes,
}

impl PreimageExistsWitness {
    pub fn new(preimage: Bytes) -> Self {
        PreimageExistsWitness { preimage }
    }
    fn to_abi(&self) -> Vec<u8> {
        ethabi::encode(&self.to_tuple())
    }
    fn to_tuple(&self) -> Vec<Token> {
        vec![Token::Bytes(self.preimage.to_vec())]
    }
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

pub struct DecisionValue {
    decision: bool,
    witness: PreimageExistsWitness,
}

impl DecisionValue {
    pub fn new(decision: bool, witness: PreimageExistsWitness) -> Self {
        DecisionValue { decision, witness }
    }
    pub fn get_decision(&self) -> bool {
        self.decision
    }
    fn to_abi(&self) -> Vec<u8> {
        ethabi::encode(&self.to_tuple())
    }
    fn to_tuple(&self) -> Vec<Token> {
        vec![
            Token::Bool(self.decision),
            Token::Bytes(self.witness.to_abi()),
        ]
    }
    fn from_tuple(tuple: &[Token]) -> Self {
        let decision = tuple[0].clone().to_bool();
        let witness = tuple[0].clone().to_bytes();
        DecisionValue::new(
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

impl PreimageExistsDecider {
    pub fn decide(&self, input: &PreimageExistsInput, witness: PreimageExistsWitness) -> Decision {
        let preimage = &witness.preimage;

        if input.verifier.hash(preimage) != input.hash {
            panic!("invalid preimage")
        }

        let decision_key = input.hash;
        let decision_value = DecisionValue::new(true, witness.clone());
        self.layer2.bucket("preimage_exists_decider").put(
            &BaseDbKey::from(decision_key.as_bytes()),
            &decision_value.to_abi(),
        );

        Decision::new(true, vec![])
    }
    pub fn check_decision(&self, input: &PreimageExistsInput) -> DecisionStatus {
        let decision_key = input.hash;
        let result = self
            .layer2
            .bucket("preimage_exists_decider")
            .get(&BaseDbKey::from(decision_key.as_bytes()));
        if let Ok(Some(decision_value_vytes)) = result {
            let decision_value = DecisionValue::from_abi(&decision_value_vytes).unwrap();
            return DecisionStatus::Decided(decision_value.get_decision());
        }
        DecisionStatus::Undecided
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Decision, PreimageExistsDecider, PreimageExistsInput, PreimageExistsWitness, Verifier,
    };
    use crate::predicate::types::DecisionStatus;
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
        assert_eq!(decided.get_outcome(), true);
        // let status = preimage_exists_decider.check_decision(&input);
        // assert_eq!(status, DecisionStatus::Decided(true));
    }

}

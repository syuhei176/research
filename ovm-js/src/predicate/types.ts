export type Bytes32 = string
export type Predicate = string

export type Property = {
  predicate: Predicate,
  input: {}
}

export interface ImplicationProofElement {
  implication: Property;
  implicationWitness: any
}

export interface Decision {
  outcome: boolean;
  implicationProof: ImplicationProofElement[];
}

export type DecisionStatus = boolean | undefined 

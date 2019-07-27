export type Bytes32 = string
export type Predicate = string

export type Input = any
export type Witness = any

export interface Property {
  predicate: Predicate
  input: Input
}

export interface ImplicationProofElement {
  implication: Property
  implicationWitness: any
}

export interface Decision {
  outcome: DecisionStatus
  implicationProof: ImplicationProofElement[]
}

export type DecisionStatus = boolean | undefined

export interface Decider {
  decide(input: Input, witness: Witness): Decision
  checkDecision(input: Input): Decision
}

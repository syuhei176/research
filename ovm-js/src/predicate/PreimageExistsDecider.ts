import { Bytes32, Decision, DecisionStatus } from './types'
import { Layer1 } from '../layer1';
import { Layer2 } from '../layer2';
import { Bucket } from '../layer2/bucket';

interface Verifier {
  hash(preimage: string): Bytes32
}

type PreimageExistsInput = {
  verifier: Verifier
  parameters: string
  hash: Bytes32
}

type PreimageExistsWitness = {
  preimage: string
}

/**
 * @description PreimageExistsDecider is simple decider which decide whether preimage exists or not
 */
export class PreimageExistsDecider {
  l1: Layer1;
  l2: Layer2;
  bucket: Bucket;
  constructor(l1: Layer1, l2: Layer2) {
    this.l1 = l1
    this.l2 = l2
    this.bucket = this.l2.bucket('preimage_exists_decider')
  }
  decide(input: PreimageExistsInput, witness: PreimageExistsWitness): Decision {
    let verify = input.verifier
    let parameters = input.parameters
    let preimage = witness.preimage

    if (verify.hash(preimage) !== input.hash) {
      throw new Error('invalid preimage')
    }

    let decisionKey = input.hash
    let decisionValue = {
      decision: true,
      witness: witness
    }
    // store and publish data
    this.bucket.put(decisionKey, JSON.stringify(decisionValue))

    return {
      outcome: true,
      implicationProof: []
    }
  }

  checkDecision(input: PreimageExistsInput): DecisionStatus {
    let decisionKey = input.hash
    // get data from local storage
    // storage is synced with other clients
    let result = this.bucket.get(decisionKey)
    if (result) {
      let decisionValue: any = JSON.parse(result)
      if (decisionValue) {
        return decisionValue.decision
      }
    }
    return undefined
  }
}

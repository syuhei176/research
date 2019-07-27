import { Bytes32, Decider, Decision, DecisionStatus } from './types'
import { Layer1 } from '../layer1'
import { Layer2 } from '../layer2'
import { Bucket } from '../layer2/bucket'

interface Verifier {
  hash(preimage: string): Bytes32
}

interface PreimageExistsInput {
  verifier: Verifier
  parameters: string
  hash: Bytes32
}

interface PreimageExistsWitness {
  preimage: string
}

/**
 * @description PreimageExistsDecider is simple decider which decide whether preimage exists or not
 */
export class PreimageExistsDecider implements Decider {
  public l1: Layer1
  public l2: Layer2
  public bucket: Bucket
  constructor(l1: Layer1, l2: Layer2) {
    this.l1 = l1
    this.l2 = l2
    this.bucket = this.l2.bucket('preimage_exists_decider')
  }
  public decide(
    input: PreimageExistsInput,
    witness: PreimageExistsWitness
  ): Decision {
    const verify = input.verifier
    const parameters = input.parameters
    const preimage = witness.preimage

    if (verify.hash(preimage) !== input.hash) {
      throw new Error('invalid preimage')
    }

    const decisionKey = input.hash
    const decisionValue = {
      decision: true,
      witness
    }
    // store and publish data
    this.bucket.put(decisionKey, JSON.stringify(decisionValue))

    return {
      outcome: true,
      implicationProof: []
    }
  }

  public checkDecision(input: PreimageExistsInput): Decision {
    const decisionKey = input.hash
    // get data from local storage
    // storage is synced with other clients
    const result = this.bucket.get(decisionKey)
    if (result) {
      const decisionValue: any = JSON.parse(result)
      if (decisionValue) {
        return {
          outcome: decisionValue.decision,
          implicationProof: [decisionValue.witness]
        }
      }
    }
    return {
      outcome: undefined,
      implicationProof: []
    }
  }
}

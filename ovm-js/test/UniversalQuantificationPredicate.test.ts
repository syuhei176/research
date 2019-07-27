import { describe, it } from 'mocha'
import { assert } from 'chai'
import { constants, utils } from 'ethers'
import { Layer1, Layer2, DeciderManager } from '../src'
import { UniversalQuantificationPredicate } from '../src/predicate/UniversalQuantificationPredicate'
import { LessThanQuantifier } from '../src/predicate/LessThanQuantifier'
import { PreimageExistsDecider } from '../src/predicate/PreimageExistsDecider'

describe('UniversalQuantificationPredicate', () => {
  it('should decide and checkDecision', () => {
    const decideManager = new DeciderManager()
    const preimageExistsDecider = new PreimageExistsDecider(
      new Layer1(),
      new Layer2()
    )
    const predicate = '1'
    decideManager.add(predicate, preimageExistsDecider)
    const preimage = utils.hexlify(utils.toUtf8Bytes('test'))
    const inputOfPreimageExitstsDecider = {
      verifier: {
        hash: (p: string) => utils.keccak256(p)
      },
      parameters: '',
      hash: utils.keccak256(preimage)
    }
    const universalQuantificationPredicate = new UniversalQuantificationPredicate(
      decideManager,
      new Layer1(),
      new Layer2()
    )
    const input = {
      quantifier: new LessThanQuantifier(),
      parameters: '',
      proposition: (_variable: any) => {
        return {
          predicate,
          input: inputOfPreimageExitstsDecider
        }
      }
    }
    const decision = universalQuantificationPredicate.decide(input)
    assert.equal(decision.outcome, true)
  })
})

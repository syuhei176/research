import { describe, it } from 'mocha'
import { assert } from 'chai'
import { constants, utils } from 'ethers'
import { Layer1, Layer2 } from '../src'
import { PreimageExistsDecider } from '../src/predicate/PreimageExistsDecider'

describe('PreimageExistsDecider', () => {
  it('should decide and checkDecision', () => {
    const preimage = utils.hexlify(utils.toUtf8Bytes('test'))
    const preimageExistsDecider = new PreimageExistsDecider(
      new Layer1(),
      new Layer2()
    )
    const input = {
      verifier: {
        hash: (p: string) => utils.keccak256(p)
      },
      parameters: '',
      hash: utils.keccak256(preimage)
    }
    const decision = preimageExistsDecider.decide(input, {
      preimage
    })
    assert.equal(decision.outcome, true)
    const status = preimageExistsDecider.checkDecision(input)
    assert.equal(status.outcome, true)
  })
})

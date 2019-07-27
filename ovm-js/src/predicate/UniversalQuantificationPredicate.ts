import { Bytes32, Decision, DecisionStatus, Property } from './types'
import { Quantifier } from './quantifier'
import { DeciderManager } from './DeciderManager'
import { Layer1 } from '../layer1'
import { Layer2 } from '../layer2'

interface UniversalQuantificationInput {
  quantifier: Quantifier
  parameters: {}
  proposition: (_variable: any) => Property
}

export class UniversalQuantificationPredicate {
  public deciderManager: DeciderManager
  public l1: Layer1
  public l2: Layer2

  constructor(deciderManager: DeciderManager, l1: Layer1, l2: Layer2) {
    this.deciderManager = deciderManager
    this.l1 = l1
    this.l2 = l2
  }

  public decide(_input: UniversalQuantificationInput): Decision {
    const quantifier = _input.quantifier
    const quantifierParameters = _input.parameters
    const quantification = quantifier.getAllQuantified(quantifierParameters)
    const results = quantification.results
    const allResultsQuantified = quantification.allResultsQuantified
    let anyUndecided = false
    for (const quantified of results) {
      // Assume variable substitution magic
      const propertyToCheck = _input.proposition(quantified)
      const predicateToCheck = this.deciderManager.getDecider(
        propertyToCheck.predicate
      )
      if (!predicateToCheck) {
        throw Error('predicateToCheck is undefined')
      }
      const inputToCheck = propertyToCheck.input
      const decision = predicateToCheck.checkDecision(inputToCheck)
      if (decision.outcome === false) {
        decision.implicationProof.push({
          implication: propertyToCheck,
          implicationWitness: quantified
        })
        return decision
      } else if (decision.outcome === undefined) {
        anyUndecided = true
      }
    }

    // Return undefined if any were undecided, or if not all the results were quantified.
    // Otherwise, we return true!
    return {
      outcome: anyUndecided || !allResultsQuantified ? undefined : true,
      implicationProof: []
    }
  }
}

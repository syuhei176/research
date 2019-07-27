import { Quantifier, QuantifierResult } from './quantifier'

type LessThanQuantifierParameters = number

export class LessThanQuantifier implements Quantifier {
  public getAllQuantified(
    _lessThanThis: LessThanQuantifierParameters
  ): QuantifierResult {
    const allQuantified = []
    for (let i = 0; i < _lessThanThis; i++) {
      allQuantified.push(i)
    }
    return {
      results: allQuantified,
      allResultsQuantified: true
    }
  }
}

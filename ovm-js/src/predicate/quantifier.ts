export interface QuantifierResult {
  results: any[]
  allResultsQuantified: boolean
}

export interface Quantifier {
  getAllQuantified(parameters: any): QuantifierResult
}

import { Decider, Predicate } from './types'

export class DeciderManager {
  private deciders: Map<Predicate, Decider> = new Map<Predicate, Decider>()
  public getDecider(predicate: Predicate): Decider | undefined {
    return this.deciders.get(predicate)
  }

  public add(predicate: Predicate, decider: Decider) {
    return this.deciders.set(predicate, decider)
  }
}

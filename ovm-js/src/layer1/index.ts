import { Transaction } from './tx'
import { Event } from './event'
import { Claim } from '../layer2/claim'

export class Layer1 {
  /**
   * @description submit
   */
  submit(tx: Transaction) {

  }

  /**
   * @description getEvents
   */
  async getEvents(): Promise<Event[]> {
    return Promise.resolve([])
  }

  /**
   * @description claimProperty
   */
  claimProperty(claim: Claim) {

  }

}

type HexString = string

/**
 * @description Bucket enable not only storing data but also sharing data with other clients.
 */
export class Bucket {
  data: Map<string, string> = new Map<string, string>()
  prefix: HexString = ''

  constructor(prefix: HexString) {
    this.prefix = prefix
  }

  /**
   * @description put
   */
  put(key: HexString, value: HexString) {
    this.data.set(this.prefix + key, value)
  }

  /**
   * @description get
   */
  get(key: HexString): HexString | undefined {
    return this.data.get(this.prefix + key)
  }

  /**
   * @description del
   */
  del(key: HexString) {
    return this.data.delete(this.prefix + key)
  }

  bucket(prefix: HexString): Bucket {
    return new Bucket(this.prefix + prefix)
  }

}

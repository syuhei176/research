import { Bucket } from './bucket'

export class Layer2 {

  bucket(prefix: string): Bucket {
    return new Bucket(prefix)
  }

}

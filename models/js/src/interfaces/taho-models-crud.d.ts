export namespace tahoModelsCrud {
  export { CrudApi }
}
export type KeyValue = Array<[string, string]> | undefined

export class CrudApi {
  constructor(config: KeyValue)
  create(query: string, data: Uint8Array): Uint8Array | undefined
  read(query: string): Array<Uint8Array> | undefined
  update(query: string, data: Uint8Array): number | undefined
  delete(query: string): number | undefined
}

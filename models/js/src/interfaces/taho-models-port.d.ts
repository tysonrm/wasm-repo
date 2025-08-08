export namespace tahoModelsPort {
  export { Ports }
}
export type Error = ErrorError
export interface ErrorError {
  tag: 'error'
  val: string
}

export class Ports {
  constructor()
  invokePort(name: string, data: Uint8Array): Uint8Array
}

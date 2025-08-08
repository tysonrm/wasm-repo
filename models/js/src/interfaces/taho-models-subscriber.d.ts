export namespace tahoModelsSubscriber {
  export { Subscription }
}
export type Topic = string

export class Subscription {
  constructor()
  subscribe(): Array<Topic>
  callback(topic: string, message: string): void
}

export namespace tahoModelsModel {
  export function getSpec(): ModelSpec
}
export type ModelName = string
export type DomainName = string
export type PortName = string
export type RelationName = string
export type WorkflowName = string
export type DatasourceName = string
export interface PortEvent {
  name: string
  wfid: WorkflowName
  state: string
}
/**
 * # Variants
 *
 * ## `"inbound"`
 *
 * ## `"outbound"`
 */
export type Directions = 'inbound' | 'outbound'
export interface Port {
  name: PortName
  adapter?: string
  direction: Directions
  consumes: Array<PortEvent>
  produces: Array<PortEvent>
  callback: PortName
  enabled: boolean
  timeout: number
  retry: number
  interval: number
  circuitBreaker: boolean
  retest: number
  undo: PortName
}
/**
 * # Variants
 *
 * ## `"one-to-many"`
 *
 * ## `"many-to-one"`
 *
 * ## `"one-to-one"`
 *
 * ## `"many-to-many"`
 */
export type Cardinalities =
  | 'one-to-many'
  | 'many-to-one'
  | 'one-to-one'
  | 'many-to-many'
export interface Relation {
  name: RelationName
  domain: DomainName
  relatedModel: ModelName
  cardinality: Cardinalities
  foreignKey: string
  localOnly: boolean
  keepRemote: boolean
  desc: string
}
export interface ModelSpec {
  name: ModelName
  domain: DomainName
  endpoint: string
  ports: Array<Port>
  relations: Array<Relation>
  datasource: DatasourceName
}
export type SpecError = SpecErrorSpecNotFound | SpecErrorSpecInvalid
export interface SpecErrorSpecNotFound {
  tag: 'spec-not-found'
  val: string
}
export interface SpecErrorSpecInvalid {
  tag: 'spec-invalid'
  val: string
}

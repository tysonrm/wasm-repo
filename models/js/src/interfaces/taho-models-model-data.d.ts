export namespace tahoModelsModelData {}
export type Primitive =
  | PrimitiveU8Type
  | PrimitiveU16Type
  | PrimitiveU32Type
  | PrimitiveU64Type
  | PrimitiveS8Type
  | PrimitiveS16Type
  | PrimitiveS32Type
  | PrimitiveS64Type
  | PrimitiveCharType
  | PrimitiveStringType
export interface PrimitiveU8Type {
  tag: 'u8-type'
  val: number
}
export interface PrimitiveU16Type {
  tag: 'u16-type'
  val: number
}
export interface PrimitiveU32Type {
  tag: 'u32-type'
  val: number
}
export interface PrimitiveU64Type {
  tag: 'u64-type'
  val: bigint
}
export interface PrimitiveS8Type {
  tag: 's8-type'
  val: number
}
export interface PrimitiveS16Type {
  tag: 's16-type'
  val: number
}
export interface PrimitiveS32Type {
  tag: 's32-type'
  val: number
}
export interface PrimitiveS64Type {
  tag: 's64-type'
  val: bigint
}
export interface PrimitiveCharType {
  tag: 'char-type'
  val: string
}
export interface PrimitiveStringType {
  tag: 'string-type'
  val: string
}
export interface DataType {
  key: string
  value: Primitive
}
export interface ModelInstance {
  uuid: string
  model: string
  domain: string
  data: Uint8Array
}

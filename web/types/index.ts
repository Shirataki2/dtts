export * as Discord from './discord'

export interface DictItem {
  order: number
  word: string
  pron: string
}

export type NumericRange<
  START extends number,
  END extends number,
  ARR extends unknown[] = [],
  ACC extends number = never,
> = ARR['length'] extends END
  ? ACC | START | END
  : NumericRange<START, END, [...ARR, 1], ARR[START] extends undefined ? ACC : ACC | ARR['length']>

export type ServerPermissionBody = {
  tag: string
  permission_bit: bigint
}

export type ServerPermission = {
  guild_id: string
} & ServerPermissionBody

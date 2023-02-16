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
  guild_id: bigint
} & ServerPermissionBody

export interface MemberPerm {
  is_mod: boolean
  perms: Record<string, boolean>
}

export type NavbarRoute = {
  name: string
  icon: string
  path: string
  type: 'route'
  sub?: boolean
  external?: boolean
  hideLogin?: boolean
  hideLogout?: boolean
  condition?: () => boolean
}

export type NavbarAction = {
  name: string
  icon: string
  action: () => void
  type: 'action'
  sub?: boolean
  hideLogin?: boolean
  hideLogout?: boolean
  condition?: () => boolean
}

export type NavbarItem = NavbarRoute | NavbarAction

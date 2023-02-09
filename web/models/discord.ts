export namespace Discord {
  export interface CurrentUser {
    id: bigint
    avatar: string
    bot: boolean
    discriminator: number
    email: string | null
    mfa_enabled: boolean
    username: string
    verified: boolean | null
    public_flags: number
  }

  export interface GuildInfo {
    id: bigint
    icon?: string
    name: string
    owner: boolean
    permissions: bigint
  }
}

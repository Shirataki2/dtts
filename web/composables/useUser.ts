import { Discord } from '@/types'
import { getApiUrl } from '@/src/utils'

export const useUser = () => {
  const user = usePersistedState<Discord.ClientUser | null>('user', null)
  const userGuilds = usePersistedState<Discord.InviteGuildResponse | null>('user.guilds', null)
  const isLoggedin = computed(() => user.value !== null)
  const available = ref(true)

  const checkAvailable = async () => {
    await fetch(getApiUrl('/')).then(res => {
      available.value = res.ok
    })
  }

  const logout = async () => {
    user.value = null
    userGuilds.value = null
    await $fetch(getApiUrl('/oauth2/logout'))
  }

  const fetchUser = async () => {
    const userData = await $fetch<Discord.ClientUser>(getApiUrl('/users/me'))
    if (userData) {
      user.value = userData
    }
  }

  const fetchUserGuilds = async () => {
    const guilds = await $fetch<Discord.InviteGuildResponse>(getApiUrl('/servers/list'))
    if (guilds) {
      userGuilds.value = guilds
    }
  }

  const userIcon = computed(() => {
    if (user.value && user.value.avatar) {
      return `https://cdn.discordapp.com/avatars/${user.value.id}/${user.value.avatar}.png`
    }
    return '/images/default_icon.png'
  })

  return {
    user,
    userIcon,
    userGuilds,
    available,
    checkAvailable,
    isLoggedin,
    logout,
    fetchUser,
    fetchUserGuilds,
  }
}

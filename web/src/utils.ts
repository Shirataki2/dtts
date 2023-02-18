import { Discord } from '@/types'

export const getApiUrl = (url: string): string => {
  // const baseUrl = process.env.API_ENDPOINT || 'http://localhost:15000'
  const baseUrl = process.client ? '/api' : 'http://api:5000'
  return `${baseUrl}${url}`
}
export const getIconUrl = (server: Discord.Guild) => {
  if (server.icon) {
    return `https://cdn.discordapp.com/icons/${server.id}/${server.icon}.png`
  } else {
    return '/images/default_icon.png'
  }
}

export const getAvatarUrl = (server: Discord.ClientUser) => {
  if (server.avatar) {
    return `https://cdn.discordapp.com/avatars/${server.id}/${server.avatar}.png`
  } else {
    return '/images/default_icon.png'
  }
}

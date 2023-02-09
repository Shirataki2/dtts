import conf from './config'
const isDev = process.env.NODE_ENV === 'development'

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  ssr: true,
  app: {
    head: {
      title: conf.application.name,
      meta: [
        { charset: 'utf-8' },
        { name: 'viewport', content: 'width=device-width, initial-scale=1' },
        { hid: 'description', name: 'description', content: conf.application.description },
        { name: 'format-detection', content: 'telephone=no,email=no' },
      ],
    },
  },
  runtimeConfig: {
    public: {
      inviteUrl: process.env.DISCORD_BOT_INVITATION_URL || 'https://discord.gg/invite',
    },
  },
  css: ['@/assets/styles/main.scss', 'vuetify/styles', '@mdi/font/css/materialdesignicons.min.css'],
  build: {
    transpile: ['vuetify'],
  },
  vite: {
    ssr: {
      noExternal: ['vuetify'],
    },
    define: {
      'process.env.DEBUG': false,
    },
  },
  modules: ['@vueuse/nuxt', '@kevinmarrec/nuxt-pwa'],
  pwa: {
    meta: {
      mobileAppIOS: true,
    },
    workbox: {
      enabled: !isDev,
    },
  },
})

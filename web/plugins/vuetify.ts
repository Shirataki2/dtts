import { defineNuxtPlugin } from '#app'
import { createVuetify } from 'vuetify'
import theme from '@/src/theme'

import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'

export default defineNuxtPlugin(nuxt => {
  const vuetify = createVuetify({
    components,
    directives,
    theme,
    ssr: true,
  })
  nuxt.vueApp.use(vuetify)
})

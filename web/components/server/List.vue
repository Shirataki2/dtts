<script setup lang="ts">
import { Discord } from '@/types'
import { useTheme } from 'vuetify'
import { getIconUrl } from '@/types/discord'
const theme = useTheme()
const conf = useRuntimeConfig()
const router = useRouter()

interface Props {
  loading?: boolean
  error?: boolean
  servers: Discord.InviteGuildResponse | null
}

const props = withDefaults(defineProps<Props>(), {
  loading: false,
  error: false,
  servers: null,
})

const isError = computed(() => {
  return props.error || (!props.loading && props.servers === null)
})

const showDivider = computed(() => {
  return (props.servers?.invited.length || 0) > 0 && (props.servers?.invitable.length || 0) > 0
})

const invite = (server: Discord.GuildInfo) => {
  const url = conf.public.inviteUrl + `&guild_id=${server.id}`
  const win = window.open(url, '_blank', 'menubar=no,toolbar=no,location=no')
  if (win) {
    const timer = setInterval(() => {
      if (win.closed) {
        clearInterval(timer)
        router.push({ path: '/servers/' + server.id.toString() })
      }
    }, 200)
  }
}
</script>

<template>
  <VCardText v-if="props.loading">
    <LoadingIndicator />
  </VCardText>
  <VCardText v-else-if="isError">
    <VAlert type="error" border="start" elevation="2" prominent> サーバー一覧の取得に失敗しました。 </VAlert>
  </VCardText>
  <VCardText v-else>
    <VList select-strategy="classic" variant="plain" :color="theme.current.value.colors['on-background']">
      <VListItem
        v-for="server in props.servers?.invited"
        :key="server.id.toString()"
        :title="server.name"
        :prepend-avatar="getIconUrl(server)"
        :to="{ path: '/servers/' + server.id.toString() }"
      >
        <template #append>
          <VBtn variant="outlined" size="small" color="green">GO</VBtn>
        </template>
      </VListItem>
      <VDivider v-if="showDivider" class="my-3" />
      <VListItem
        v-for="server in props.servers?.invitable"
        :key="server.id.toString()"
        :title="server.name"
        :prepend-avatar="getIconUrl(server)"
        @click="invite(server)"
      >
        <template #append>
          <VBtn variant="outlined" size="small" color="red">招待</VBtn>
        </template>
      </VListItem>
    </VList>
  </VCardText>
</template>

<style lang="scss" scoped>
.v-list-item--variant-plain {
  opacity: 0.95;
}
</style>

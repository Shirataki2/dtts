<script setup lang="ts">
import config from '@/config'
const { fetchUser, logout, checkAvailable, available } = useUser()
const { showSnackbar } = useSnackbar()

useHead({
  titleTemplate: titleChunk => {
    return titleChunk ? `${titleChunk} - ${config.application.name}` : config.application.name
  },
})

onMounted(async () => {
  await checkAvailable()
  if (!available.value) {
    showSnackbar({
      text: 'APIサーバーが起動していないため、サービスを利用できません。',
      color: 'error',
      timeout: 150_000,
    })
  }
  try {
    await fetchUser()
  } catch {
    logout()
  }
})
</script>

<template>
  <NuxtLayout>
    <NuxtPage />
  </NuxtLayout>
</template>

<style lang="scss">
@import '@/assets/styles/global.scss';
</style>

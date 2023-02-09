<script setup lang="ts">
useHead({
  title: 'サーバー一覧',
})

definePageMeta({
  middleware: ['auth'],
})

const { userGuilds, fetchUserGuilds } = useUser()
const loading = ref(true)

onMounted(async () => {
  await fetchUserGuilds()
  loading.value = false
})
</script>

<template>
  <LayoutContained variant="compact">
    <VCard>
      <VCardTitle class="text-center">
        <h1 class="text-h6">サーバーを選択してください</h1>
      </VCardTitle>
      <ServerList :loading="loading" :servers="userGuilds" />
    </VCard>
  </LayoutContained>
</template>

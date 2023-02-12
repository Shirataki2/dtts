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
      <PageTitle>サーバー一覧</PageTitle>
      <ServerList :loading="loading" :servers="userGuilds" />
    </VCard>
  </LayoutContained>
</template>

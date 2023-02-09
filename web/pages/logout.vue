<script setup lang="ts">
useHead({
  title: 'ログアウト',
})
const { logout } = useUser()

onMounted(async () => {
  await logout()
  window.location.href = '/'
})
</script>

<template>
  <LayoutCentered>
    <LoadingIndicator />
    <p class="text-h6 text-centered">ログアウトしています...</p>
  </LayoutCentered>
</template>

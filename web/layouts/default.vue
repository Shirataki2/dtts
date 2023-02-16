<script setup lang="ts">
// c.f. https://github.com/vuetifyjs/vuetify/issues/15202
const paddingSpaceOnLoad = ref(true)

onMounted(() => {
  paddingSpaceOnLoad.value = false
})
</script>

<template>
  <VApp>
    <AppHeader />
    <AppNavDrawer />
    <div v-if="paddingSpaceOnLoad" style="height: 56px" />
    <VMain>
      <slot />
    </VMain>
    <AppFooter />
    <AppDialog />
    <AppSnackbar />
  </VApp>
</template>

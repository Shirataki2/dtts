<script setup lang="ts">
useHead({
  title: 'Index',
})
const { user } = useUser()
</script>

<template>
  <LayoutContained>
    <h1>Index page</h1>
    <p>{{ user }}</p>
    <VBtn variant="elevated" color="primary">Hello World</VBtn>
    <VBtn variant="elevated" color="secondary">Hello World</VBtn>
    <VBtn variant="elevated" color="error">Hello World</VBtn>
    <VBtn variant="elevated" color="warning">Hello World</VBtn>
    <VBtn variant="elevated" color="success">Hello World</VBtn>
    <VBtn variant="elevated" color="info">Hello World</VBtn>
  </LayoutContained>
</template>

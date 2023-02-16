<script setup lang="ts">
interface Props {
  type?: 'error' | 'success' | 'info' | 'warning'
}

const props = withDefaults(defineProps<Props>(), {
  type: 'error',
})
</script>

<template>
  <VAlert border="start" variant="outlined" :type="props.type" class="my-5">
    <strong><slot /></strong>
  </VAlert>
</template>

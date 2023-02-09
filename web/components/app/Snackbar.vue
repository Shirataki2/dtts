<script setup lang="ts">
const { snackbar } = useSnackbar()
const emit = defineEmits<{
  (e: 'update:modelValue', v: boolean): void
}>()
const show = computed({
  get: () => snackbar.value !== null,
  set: v => {
    if (!v) snackbar.value = null
    emit('update:modelValue', v)
  },
})
</script>

<template>
  <VSnackbar v-model="show" :color="snackbar?.color" :timeout="snackbar?.timeout">
    {{ snackbar?.text }}
    <template #actions>
      <VBtn variant="text" @click="show = false">Close</VBtn>
    </template>
  </VSnackbar>
</template>

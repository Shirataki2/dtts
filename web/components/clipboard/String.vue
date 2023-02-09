<script setup lang="ts">
import { useClipboard } from '@vueuse/core'
const { showSnackbar } = useSnackbar()

interface Props {
  value: string
  label?: string
  successText?: string
  errorText?: string
  textSize?: string
}

const props = withDefaults(defineProps<Props>(), {
  successText: 'コピーしました',
  errorText: 'コピーに失敗しました',
  textSize: '16px',
})

const { copy, copied, isSupported } = useClipboard({ source: props.value })

const copyValue = async () => {
  try {
    await copy(props.value)
    showSnackbar({
      text: props.successText,
      color: 'success',
      timeout: 1500,
    })
  } catch (e) {
    showSnackbar({
      text: props.errorText,
      color: 'error',
      timeout: 1500,
    })
  }
}
</script>

<template>
  <span class="copy mr-1" v-if="props.label">{{ props.label }}: </span>
  <span class="copy">{{ props.value }}</span>
  <VBtn v-if="isSupported" size="x-small" @click="copyValue" variant="text">
    <VIcon v-if="!copied">mdi-content-copy</VIcon>
    <VIcon v-else>mdi-check</VIcon>
  </VBtn>
</template>

<style scoped>
.copy {
  font-size: v-bind('props.textSize');
}
</style>

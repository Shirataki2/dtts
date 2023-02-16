<script setup lang="ts">
import { useDialog } from '@/composables/useDialog'

const { dialog, result } = useDialog()
</script>

<template>
  <VDialog :model-value="dialog !== null" persistent>
    <VCard class="mx-auto" :max-width="dialog?.width || '100%'">
      <VCardTitle class="text-center">
        <h1 class="text-h6">{{ dialog?.title }}</h1>
      </VCardTitle>
      <VCardText>
        {{ dialog?.description }}
      </VCardText>
      <VCardActions>
        <VSpacer />
        <VBtn
          variant="text"
          v-for="action in dialog?.actions"
          :color="action.color || 'on-background'"
          :key="action.text"
          @click="
            () => {
              result = action.handler()
              dialog = null
            }
          "
        >
          {{ action.text }}
        </VBtn>
      </VCardActions>
    </VCard>
  </VDialog>
</template>

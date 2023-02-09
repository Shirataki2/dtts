<script setup lang="ts">
type ButtonVariant = 'flat' | 'outlined' | 'tonal' | 'elevated' | 'text' | 'plain'
interface Props {
  msg: string
  variant?: ButtonVariant
  color?: string
}
withDefaults(defineProps<Props>(), {
  msg: 'Hello World',
  variant: 'flat',
  color: 'primary',
})
</script>

<template>
  <VBtn :variant="variant">{{ msg }}</VBtn>
</template>

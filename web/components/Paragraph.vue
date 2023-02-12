<script setup lang="ts">
import { NumericRange } from '@/types'
type ParagraphVariant = NumericRange<0, 6>
interface Props {
  margin?: ParagraphVariant
}
const props = withDefaults(defineProps<Props>(), {
  margin: 4,
})
const classes = computed(() => {
  return {
    'mx-2': true,
    [`my-${props.margin * 2}`]: true,
  }
})
</script>

<template>
  <p :class="classes">
    <slot />
  </p>
</template>

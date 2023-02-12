<script setup lang="ts">
type LayoutVariant = 'full-width' | 'wide' | 'standard' | 'compact'
interface Props {
  variant?: LayoutVariant
}
const props = withDefaults(defineProps<Props>(), {
  variant: 'standard',
})

const columnProps = computed(() => {
  if (props.variant === 'full-width') {
    return {
      class: 'w-full text-center',
    }
  } else if (props.variant === 'wide') {
    return {
      offsetLg: 1,
      lg: 10,
      offsetXl: 2,
      xl: 8,
      class: 'text-center',
    }
  } else if (props.variant === 'standard') {
    return {
      offsetSm: 1,
      sm: 10,
      offsetLg: 2,
      lg: 8,
      offsetXl: 3,
      xl: 6,
      class: 'text-center',
    }
  } else if (props.variant === 'compact') {
    return {
      offsetSm: 1,
      sm: 10,
      offsetLg: 3,
      lg: 6,
      offsetXl: 4,
      xl: 4,
      class: 'text-center',
    }
  }
  return {}
})
</script>

<template>
  <VContainer>
    <VRow>
      <VCol v-bind="columnProps">
        <slot />
      </VCol>
    </VRow>
  </VContainer>
</template>

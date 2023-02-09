<script setup lang="ts">
interface Props {
  tip?: string
  icon: string
  disabled?: boolean
  color?: string
}

const props = withDefaults(defineProps<Props>(), {
  disabled: false,
})

const emits = defineEmits(['click'])
</script>

<template>
  <VTooltip location="top">
    <template #activator="{ props: toolProps }">
      <VBtn
        variant="text"
        :color="props.color || 'on-background'"
        v-bind="toolProps"
        :disabled="props.disabled"
        :icon="props.icon"
        @click="emits('click')"
        class="mx-1"
      />
    </template>
    <span>{{ props.tip }}</span>
  </VTooltip>
</template>

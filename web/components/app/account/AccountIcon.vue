<script setup lang="ts">
const { userIcon } = useUser()
interface Props {
  size?: number
}

const props = withDefaults(defineProps<Props>(), {
  size: 42,
})
</script>

<template>
  <VAvatar :size="props.size">
    <VImg :src="userIcon" alt="User Icon" />
  </VAvatar>
</template>

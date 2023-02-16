<script lang="ts" setup>
interface Props {
  hideButton?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  hideButton: false,
})

const router = useRouter()

const back = () => {
  router.back()
}
</script>

<template>
  <VCardTitle class="text-center mt-1 mb-3">
    <VBtn v-if="!props.hideButton" variant="text" icon @click="back" id="back-btn">
      <VIcon>mdi-chevron-left</VIcon>
    </VBtn>
    <h1 class="text-h6"><slot /></h1>
  </VCardTitle>
</template>

<style scoped>
#back-btn {
  position: absolute;
  left: 0;
  top: 0;
  margin: 0 1rem;
}
</style>

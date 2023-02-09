<script setup lang="ts">
interface Props {
  code: object
  prettify?: boolean
  fontSize?: string
}

const props = withDefaults(defineProps<Props>(), {
  prettify: false,
  fontSize: '14px',
})

const code = computed(() => {
  if (props.prettify) {
    return prettify(props.code)
  }
  return props.code
})

const prettify = (code: object) => {
  try {
    return JSON.stringify(code, null, 2)
  } catch (e) {
    return code
  }
}
</script>

<template>
  <DevOnly>
    <VDivider class="my-3" />
    <p class="mb-2">開発用データ表示領域</p>
    <pre>
<code>{{ code }}</code>
    </pre>
    <VDivider class="my-3" />
  </DevOnly>
</template>

<style scoped>
pre {
  background: #111;
  padding: 1rem;
  border-radius: 0.5rem;
  margin: 0;
  overflow: auto;
  white-space: pre-wrap;
  font-size: v-bind('props.fontSize');
}
code {
  color: #f3f3f3;
}
</style>

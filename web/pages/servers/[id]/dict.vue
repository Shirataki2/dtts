<script setup lang="ts">
import { useRefHistory } from '@vueuse/core'
import { klona } from 'klona'
import { getApiUrl } from '@/src/utils'

definePageMeta({
  middleware: ['auth'],
})

interface DictItem {
  order: number
  word: string
  pron: string
}

const { params } = useRoute()
const id = typeof params.id === 'string' ? params.id : params.id[0]
const { data: dict, pending } = useFetch<DictItem[]>(getApiUrl(`/servers/dict?id=${id}`))
const { history: dictHistory, undo, redo } = useRefHistory(dict, { dump: klona })
useHead({
  title: '辞書編集',
})
</script>

<template>
  <LayoutContained variant="wide" v-if="!pending && dict">
    <VCard>
      <VCardTitle class="text-center">
        <h1 class="text-h6">辞書編集</h1>
      </VCardTitle>
      <DictToolbar />
      <DevShowCode :code="dict || {}" :prettify="false" />
    </VCard>
  </LayoutContained>
  <LayoutCentered v-else>
    <VProgressCircular indeterminate />
    <p class="my-5">辞書情報を取得中...</p>
  </LayoutCentered>
</template>

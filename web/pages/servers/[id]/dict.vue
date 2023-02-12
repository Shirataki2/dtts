<script setup lang="ts">
import { useManualRefHistory } from '@vueuse/core'
import { klona } from 'klona'
import { getApiUrl } from '@/src/utils'
import { DictItem } from '@/types'

const { showSnackbar } = useSnackbar()
const show = ref(false)

definePageMeta({
  middleware: ['auth'],
})

const { params } = useRoute()
const id = typeof params.id === 'string' ? params.id : params.id[0]
const { data: dictRef, pending, execute } = useFetch<DictItem[]>(getApiUrl(`/servers/dict?id=${id}`))
await execute()
if (typeof dictRef.value === 'undefined') {
  showSnackbar({
    text: '辞書情報の取得に失敗しました',
    color: 'error',
  })
  dictRef.value = []
}
// dict is now a Ref<DictItem[]>
const dict = <Ref<DictItem[]>>dictRef
const { history: dictHistory, undo, redo, commit, canUndo, canRedo, clear } = useManualRefHistory(dict, { dump: klona })
useHead({
  title: '辞書編集',
})

onMounted(() => {
  show.value = true
})
</script>

<template>
  <LayoutContained variant="wide" v-if="!pending && show">
    <VCard>
      <VCardTitle class="text-center">
        <h1 class="text-h6">辞書編集</h1>
      </VCardTitle>
      <DictToolbar
        :dict="dict"
        :dict-history="dictHistory"
        @undo="undo"
        @redo="redo"
        @commit="commit"
        @clear="clear"
        :can-undo="canUndo"
        :can-redo="canRedo"
      />
      <DictForm :dict="dict" @undo="undo" @redo="redo" @change="commit" />
      <DevShowCode :code="dict || {}" :prettify="false" />
    </VCard>
  </LayoutContained>
  <LayoutCentered v-else>
    <VProgressCircular indeterminate />
    <p class="my-5">辞書情報を取得中...</p>
  </LayoutCentered>
</template>

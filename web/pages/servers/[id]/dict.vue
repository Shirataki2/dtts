<script setup lang="ts">
import { useManualRefHistory } from '@vueuse/core'
import { klona } from 'klona'
import { getApiUrl } from '@/src/utils'
import { DictItem, MemberPerm } from '@/types'

const { showSnackbar } = useSnackbar()
const { showDialog, close } = useDialog<boolean>()
const show = ref(false)

definePageMeta({
  middleware: ['auth'],
})

const { params } = useRoute()
const id = typeof params.id === 'string' ? params.id : params.id[0]
const { data: dictRef, execute } = useFetch<DictItem[]>(getApiUrl(`/servers/dict?id=${id}`))
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

const initialDict = ref(klona(dict.value))

const isDirty = computed(() => {
  return JSON.stringify(dict.value) !== JSON.stringify(initialDict.value)
})

useConfirm(isDirty)

const { history: dictHistory, undo, redo, commit, canUndo, canRedo, clear } = useManualRefHistory(dict, { dump: klona })
useHead({
  title: '辞書編集',
})

const canEdit = computed(() => {
  if (perms.value) {
    return perms.value.perms['dict.edit']
  }
})

const { data: perms, pending } = useFetch<MemberPerm>(getApiUrl('/servers/perms/check'), {
  params: {
    id: id,
  },
})

onMounted(() => {
  show.value = true
})

const afterSave = () => {
  initialDict.value = klona(dict.value)
}
</script>

<template>
  <LayoutContained variant="wide" v-if="!pending && show">
    <VCard>
      <PageTitle>辞書編集</PageTitle>
      <VCardText>
        <AppAlert v-if="!canEdit" type="warning"> サーバー管理者によって辞書の編集が制限されています </AppAlert>
        <DictToolbar
          v-if="canEdit"
          :dict="dict"
          :dict-history="dictHistory"
          @undo="undo"
          @redo="redo"
          @commit="commit"
          @clear="clear"
          @after-save="afterSave"
          :can-undo="canUndo"
          :can-redo="canRedo"
        />
        <DictForm :disabled="!canEdit" :dict="dict" @undo="undo" @redo="redo" @change="commit" />
        <DevShowCode :code="dict || {}" :prettify="false" />
      </VCardText>
    </VCard>
  </LayoutContained>
  <LayoutCentered variant="wide" v-else>
    <VCard>
      <LoadingCircular>辞書情報を取得中...</LoadingCircular>
    </VCard>
  </LayoutCentered>
</template>

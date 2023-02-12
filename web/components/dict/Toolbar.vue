<script setup lang="ts">
import { DictItem } from '@/types'
import { UseRefHistoryRecord } from '@vueuse/core'
import { klona } from 'klona'
import { getApiUrl } from '~~/src/utils'

interface Props {
  dict: DictItem[]
  dictHistory: UseRefHistoryRecord<DictItem[]>[]
  canUndo: boolean
  canRedo: boolean
}

const { params } = useRoute()
const { showSnackbar } = useSnackbar()
const { validate, saving } = useValidate('dict')
const { showDialog, close } = useDialog<boolean>()
const props = defineProps<Props>()
const initialDict = ref(klona(props.dict))
const canSave = ref(true)

const emits = defineEmits<{
  (e: 'undo'): void
  (e: 'redo'): void
  (e: 'commit'): void
  (e: 'clear'): void
}>()

const isDirty = computed(() => {
  return JSON.stringify(props.dict) !== JSON.stringify(initialDict.value)
})

const addRow = () => {
  if (props.dict.length === 0) {
    props.dict.push({
      order: 0,
      word: '',
      pron: '',
    })
    emits('commit')
    return
  }
  let max_order = props.dict.reduce((a, b) => (a.order > b.order ? a : b)).order
  props.dict.push({
    order: max_order + 1,
    word: '',
    pron: '',
  })
  emits('commit')
}

const loadDictFromFile = () => {
  const el = document.getElementById('file-input') as HTMLInputElement
  el.click()
}

const saveDictToFile = () => {
  const kvPairs = props.dict.map(item => `${item.word}\t${item.pron}`)
  const text = kvPairs.join('\n')
  const blob = new Blob([text], { type: 'text/plain' })
  const fileName = `dict_${params.id}.dict`
  const navigator = window.navigator as any
  if (navigator.msSaveOrOpenBlob) {
    navigator.msSaveOrOpenBlob(blob, fileName)
  } else {
    const a = document.createElement('a')
    a.href = URL.createObjectURL(blob)
    a.download = fileName
    a.click()
    setTimeout(() => {
      URL.revokeObjectURL(a.href)
    }, 0)
  }
}

const saveDictToServer = async () => {
  const isValid = await validate.value()
  if (!isValid) {
    return showSnackbar({
      text: '辞書にエラーがあります',
      color: 'error',
    })
  }
  try {
    saving.value = true
    canSave.value = false
    await $fetch(getApiUrl('/servers/dict'), {
      method: 'POST',
      params: {
        id: params.id,
      },
      body: props.dict,
    })
    showSnackbar({
      text: '辞書を保存しました',
      color: 'success',
    })
    emits('clear')
    initialDict.value = klona(props.dict)
  } catch (e) {
    console.error(e)
    showSnackbar({
      text: '辞書の保存に失敗しました',
      color: 'error',
    })
  } finally {
    saving.value = false
    // To prevent double-clicking and spamming the server
    await new Promise(resolve => setTimeout(resolve, 10_000))
    canSave.value = true
  }
}

const resetDict = async () => {
  const result = await showDialog({
    title: '辞書をリセットしますか？',
    description: '辞書をリセットすると、現在の辞書の内容がすべて消えます。',
    width: '600px',
    actions: [
      {
        text: 'キャンセル',
        color: 'primary',
        handler: () => {
          close()
          return false
        },
      },
      {
        text: 'リセット',
        color: 'error',
        handler: () => {
          close()
          return true
        },
      },
    ],
  })
  if (result.value) {
    if (initialDict.value.length === 0) {
      props.dict.splice(0, props.dict.length)
      addRow()
    } else {
      props.dict.splice(0, props.dict.length, ...initialDict.value)
    }
    emits('commit')
  }
}

const readFromFile = (e: Event) => {
  const file = (e.target as HTMLInputElement).files?.[0]
  if (!file) {
    return
  }
  const reader = new FileReader()
  reader.onload = e => {
    const text = e.target?.result
    if (typeof text !== 'string') {
      return
    }
    const dict = []
    const lines = text.split('\n')
    let order = 0
    for (const line of lines) {
      const kvPair = line.split('\t')
      if (kvPair.length !== 2) {
        showSnackbar({
          text: `不正な行があります: ${line}`,
          color: 'error',
        })
        return
      }
      const [word, pron] = kvPair
      dict.push({
        order: order++,
        word,
        pron,
      })
    }
    props.dict.splice(0, props.dict.length, ...dict)
    emits('commit')
  }
  reader.readAsText(file)
}
</script>

<template>
  <VContainer>
    <VRow>
      <VCol cols="12" sm="6">
        <DictToolButton :disabled="!canUndo || saving" icon="mdi-restore" tip="元に戻す" @click="emits('undo')" />
        <DictToolButton :disabled="!canRedo || saving" icon="mdi-reload" tip="やり直す" @click="emits('redo')" />
        <DictToolButton :disabled="saving" icon="mdi-plus" tip="行を追加" @click="addRow" />
        <DictToolButton :disabled="saving" icon="mdi-import" tip="ファイルから読み込む" @click="loadDictFromFile">
          <input id="file-input" type="file" style="display: none" @change="readFromFile" />
        </DictToolButton>
        <DictToolButton :disabled="saving" icon="mdi-export" tip="ファイルに保存" @click="saveDictToFile" />
      </VCol>
      <VCol cols="12" sm="6" class="text-right">
        <DictToolButton
          :disabled="!isDirty || saving || !canSave"
          color="primary"
          icon="mdi-content-save"
          tip="保存"
          @click="saveDictToServer"
        />
        <DictToolButton
          :disabled="!isDirty || saving"
          color="error"
          icon="mdi-trash-can"
          tip="リセット"
          @click="resetDict"
        />
      </VCol>
    </VRow>
    <DevShowCode :code="props.dict" />
    <DevShowCode :code="initialDict" />
  </VContainer>
</template>

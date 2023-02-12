<script setup lang="ts">
import { DictItem } from '@/types'
import { VForm } from 'vuetify/components'

interface Props {
  dict: DictItem[]
  disabled?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  disabled: false,
})

const emits = defineEmits<{
  (e: 'change'): void
  (e: 'undo'): void
  (e: 'redo'): void
}>()

const rules = {
  required: (value: string) => !!value || '必須項目です。',
  maxLength: (len: number) => (value: string) => value.length <= len || `${len}文字以内で入力してください。`,
}

const { validate, saving } = useValidate('dict')
const form = ref<VForm>()

const onEnter = async (item: DictItem) => {
  const isLastItem = props.dict[props.dict.length - 1].order === item.order
  const wait = (ms: number) => new Promise(resolve => setTimeout(resolve, ms))
  if (isLastItem) {
    const max_order = props.dict.reduce((a, b) => (a.order > b.order ? a : b)).order
    props.dict.push({
      order: max_order + 1,
      word: '',
      pron: '',
    })
    emits('change')
    // 一番下の行にフォーカスを移動する
    // await nextTick()だと動かなかったため便宜的にsetTimeoutを使っている
    await wait(3)
    const lastItem = props.dict[props.dict.length - 1]
    const el = document.getElementById(`word_${lastItem.order}`)
    if (el) {
      el.focus()
    }
  } else {
    const currentFocus = document.activeElement as HTMLElement
    const currentFocusId = currentFocus.id
    const currentFocusOrder = Number(currentFocusId.split('_')[1])
    const nextFocusOrder = props.dict[props.dict.findIndex(i => i.order === currentFocusOrder)].order + 1
    if (currentFocusId.startsWith('pron_')) {
      const nextFocusId = `word_${nextFocusOrder}`
      const nextFocus = document.getElementById(nextFocusId)
      if (nextFocus) {
        nextFocus.focus()
      }
    }
  }
}

const deleteRow = (item: DictItem) => {
  const index = props.dict.findIndex(i => i.order === item.order)
  props.dict.splice(index, 1)
  emits('change')
}

validate.value = async () => {
  console.log('validate')
  if (form.value) {
    return (await form.value.validate()).valid
  } else {
    return true
  }
}
</script>

<template>
  <VContainer>
    <VRow>
      <VCol cols="12">
        <VForm ref="form" :disabled="saving">
          <VTable>
            <thead>
              <tr>
                <th width="46%">単語</th>
                <th width="46%">読み方</th>
                <th width="8%">操作</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="item in props.dict" :key="item.order">
                <td>
                  <VTextField
                    :id="`word_${item.order}`"
                    class="mt-3"
                    v-model="item.word"
                    density="compact"
                    variant="outlined"
                    :rules="[rules.required, rules.maxLength(10)]"
                    @update:model-value="emits('change')"
                    @keyup.ctrl.z="emits('undo')"
                    @keyup.ctrl.y="emits('redo')"
                  />
                </td>
                <td>
                  <VTextField
                    :id="`pron_${item.order}`"
                    class="mt-3"
                    v-model="item.pron"
                    density="compact"
                    variant="outlined"
                    :rules="[rules.required, rules.maxLength(30)]"
                    @update:model-value="emits('change')"
                    @keyup.ctrl.z="emits('undo')"
                    @keyup.ctrl.y="emits('redo')"
                    @keydown.enter="onEnter(item)"
                  />
                </td>
                <td>
                  <DictToolButton :id="`delete_${item.order}`" icon="mdi-delete" tip="削除" @click="deleteRow(item)" />
                </td>
              </tr>
            </tbody>
          </VTable>
        </VForm>
      </VCol>
    </VRow>
  </VContainer>
</template>

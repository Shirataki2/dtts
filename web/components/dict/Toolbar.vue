<script setup lang="ts">
const undo = () => {
  console.log('undo')
}

const redo = () => {
  console.log('redo')
}

const addRow = () => {
  console.log('addRow')
}

const loadDictFromFile = () => {
  console.log('loadDictFromFile')
}

const saveDictToFile = () => {
  console.log('saveDictToFile')
}
</script>

<template>
  <VContainer>
    <VRow>
      <VCol cols="12" sm="6">
        <DictToolButton icon="mdi-restore" tip="元に戻す" @click="undo" />
        <DictToolButton icon="mdi-reload" tip="やり直す" @click="redo" />
        <DictToolButton icon="mdi-plus" tip="行を追加" @click="addRow" />
        <DictToolButton icon="mdi-import" tip="ファイルから読み込む" @click="loadDictFromFile" />
        <DictToolButton icon="mdi-export" tip="ファイルに保存" @click="saveDictToFile" />
      </VCol>
    </VRow>
  </VContainer>
</template>

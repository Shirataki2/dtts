<script setup lang="ts">
import { klona } from 'klona'
import { ServerPermission, ServerPermissionBody } from '@/types'
interface Props {
  perms: ServerPermission[]
  disabled?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  disabled: false,
})

const emits = defineEmits<{
  (e: 'save', perms: ServerPermissionBody[]): Promise<void>
}>()

const userPermissions = [
  { label: 'チャンネル閲覧', bit: 2 ** 10 },
  { label: 'メッセージ送信', bit: 2 ** 11 },
  { label: 'コマンドの使用', bit: 2 ** 31 },
  { label: 'メッセージ管理', bit: 2 ** 13 },
  { label: 'ロール管理', bit: 2 ** 28 },
]

const servicePermissions = [
  { label: '辞書編集', tag: 'dict.edit' },
  { label: '読み上げ設定', tag: 'tts.settings.edit' },
]

const W = userPermissions.length

const permsToMatrix = (perms: ServerPermission[]) => {
  const matrix: Array<boolean> = Array(servicePermissions.length * userPermissions.length).fill(false)
  for (const perm of perms) {
    const row = servicePermissions.findIndex(row => row.tag === perm.tag)
    const cols = userPermissions.map(col => (BigInt(perm.permission_bit) & BigInt(col.bit)) > 0)
    for (let x = 0; x < cols.length; x++) {
      matrix[row * W + x] = cols[x]
    }
  }
  return matrix
}

const matrix = ref(permsToMatrix(klona(props.perms)))

const matrixToPerms = (matrix: Array<boolean>) => {
  const perms: ServerPermissionBody[] = []
  for (let y = 0; y < servicePermissions.length; y++) {
    let bit = BigInt(0)
    for (let x = 0; x < userPermissions.length; x++) {
      if (matrix[y * W + x]) {
        bit |= BigInt(userPermissions[x].bit)
      }
      perms.push({
        tag: servicePermissions[y].tag,
        permission_bit: bit,
      })
    }
  }
  return perms
}

const saving = ref(false)
const { showSnackbar } = useSnackbar()

const onSave = async () => {
  saving.value = true
  const perms = matrixToPerms(matrix.value)
  try {
    await emits('save', perms)
    showSnackbar({
      text: '保存しました',
      color: 'success',
    })
  } catch (e) {
    showSnackbar({
      text: '保存に失敗しました',
      color: 'error',
    })
  } finally {
    saving.value = false
  }
}
</script>

<template>
  <VTable>
    <tr>
      <th />
      <th v-for="userPermission in userPermissions" :key="userPermission.label" class="user-permission-header">
        {{ userPermission.label }}
      </th>
    </tr>
    <tr v-for="(servicePermission, y) in servicePermissions" :key="servicePermission.label">
      <th>{{ servicePermission.label }}</th>
      <td v-for="(userPermission, x) in userPermissions" :key="userPermission.label" class="checkbox">
        <VCheckbox
          :disabled="disabled || saving"
          v-model="matrix[y * W + x]"
          class="align-center justify-center"
          density="compact"
          hide-details
        />
      </td>
    </tr>
  </VTable>
  <VBtn v-if="!disabled" :disabled="saving" variant="outlined" block color="success" class="mt-5" @click="onSave">
    保存
  </VBtn>
</template>

<style scoped>
.user-permission-header {
  writing-mode: vertical-rl;
}
</style>

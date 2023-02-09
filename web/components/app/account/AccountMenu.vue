<script setup lang="ts">
const { themeLabel, rotateTheme } = useAppTheme()
const router = useRouter()

const menus = [
  {
    key: 'servers',
    name: () => 'サーバー一覧',
    callback: () => {
      router.push({ name: 'servers' })
    },
  },
  {
    key: 'account',
    name: () => 'アカウント',
    callback: () => {},
  },
  {
    key: 'logout',
    name: () => 'ログアウト',
    callback: () => {
      router.push({ name: 'logout' })
    },
  },
  {
    key: 'switchTheme',
    name: () => `テーマ: ${themeLabel.value}`,
    callback: rotateTheme,
  },
]
</script>

<template>
  <VMenu offset="bottom">
    <template #activator="{ props }">
      <VBtn icon v-bind="props">
        <slot>
          <VIcon>mdi-account</VIcon>
        </slot>
      </VBtn>
    </template>
    <VList density="compact" width="209px">
      <VListItem v-for="menu in menus" :key="menu.key" :value="menu.key">
        <VListItemTitle @click="menu.callback">
          {{ menu.name() }}
        </VListItemTitle>
      </VListItem>
    </VList>
  </VMenu>
</template>

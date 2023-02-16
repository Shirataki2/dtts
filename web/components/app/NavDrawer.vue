<script setup lang="ts">
import { NavbarItem } from '@/types'

const config = useRuntimeConfig()
const { drawer } = useNavDrawer()
const { isLoggedin } = useUser()
const { themeLabel, rotateTheme } = useAppTheme()

const items = computed<NavbarItem[]>(() => [
  {
    name: 'ホーム',
    icon: 'mdi-home',
    path: '/',
    type: 'route',
  },
  {
    name: 'サーバー一覧',
    icon: 'mdi-format-list-bulleted-square',
    path: '/servers',
    type: 'route',
    hideLogout: true,
  },
  {
    name: 'アカウント',
    icon: 'mdi-account',
    path: '/account',
    type: 'route',
    hideLogout: true,
  },
  {
    name: 'ログイン',
    icon: 'mdi-login',
    path: '/login',
    type: 'route',
    hideLogin: true,
  },
  {
    name: '寄付',
    icon: 'mdi-gift',
    path: '/donate',
    type: 'route',
    hideLogout: true,
  },
])

const themeIcon = computed(() => {
  if (themeLabel.value === 'ライト') {
    return 'mdi-white-balance-sunny'
  } else if (themeLabel.value === 'ダーク') {
    return 'mdi-weather-night'
  } else {
    return 'mdi-laptop'
  }
})

const bottomItems = computed<NavbarItem[]>(() => [
  {
    name: 'ログアウト',
    icon: 'mdi-logout',
    path: '/logout',
    type: 'route',
    hideLogout: true,
  },
  {
    name: 'GitHub',
    icon: 'mdi-github',
    path: config.public.repositoryUrl,
    type: 'route',
    external: true,
  },
  {
    name: 'テーマ: ' + themeLabel.value,
    icon: themeIcon.value,
    type: 'action',
    action: rotateTheme,
  },
])

const filterItems = (items: NavbarItem[]) => {
  return items.filter(item => {
    if (item.hideLogin && isLoggedin.value) return false
    if (item.hideLogout && !isLoggedin.value) return false
    if (item.condition && !item.condition()) return false
    return true
  })
}
</script>

<template>
  <VNavigationDrawer v-model="drawer">
    <VList>
      <AppNavDrawerItem v-for="item in filterItems(items)" :key="item.name" :item="item" />
    </VList>
    <template #append>
      <VList>
        <AppNavDrawerItem v-for="item in filterItems(bottomItems)" :key="item.name" :item="item" />
      </VList>
    </template>
  </VNavigationDrawer>
</template>

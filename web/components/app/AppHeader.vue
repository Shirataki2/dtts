<script setup lang="ts">
import config from '@/config'
const { toggleDrawer } = useNavDrawer()

const { isLoggedin } = useUser()
</script>

<template>
  <VAppBar app flat density="comfortable" color="header" style="width: 100%">
    <VAppBarNavIcon @click.stop="toggleDrawer" />
    <VToolbarTitle class="pointer" @click="$router.push({ name: 'index' })">
      <span class="font-weight-bold">{{ config.application.name }}</span>
    </VToolbarTitle>
    <VSpacer />
    <template v-if="isLoggedin">
      <AppAccountMenu>
        <AppAccountIcon />
      </AppAccountMenu>
    </template>
    <template v-else>
      <VBtn text href="/api/oauth2/login">Login</VBtn>
    </template>
  </VAppBar>
</template>

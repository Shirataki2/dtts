<script setup lang="ts">
import { Discord } from '@/types'
import { getIconUrl } from '~~/src/utils'

interface Props {
  server: Discord.Guild
}

const props = defineProps<Props>()
</script>

<template>
  <VCardTitle>
    <VContainer>
      <VRow id="server_name">
        <VCol cols="4" sm="3" md="2">
          <VAvatar size="100%">
            <VImg :src="getIconUrl(props.server)" />
          </VAvatar>
        </VCol>
        <VCol cols="8" sm="9" md="10">
          <h1 class="text-h5">{{ props.server.name }}</h1>
          <ClipboardString label="ID" :value="props.server.id" />
        </VCol>
      </VRow>
    </VContainer>
  </VCardTitle>
</template>

<style lang="scss">
#server_name {
  margin-top: 0.5rem;
  margin-bottom: 0.5rem;
  align-items: center;
}
</style>

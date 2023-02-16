<script setup lang="ts">
import { Discord } from '@/types'
import { getApiUrl } from '@/src/utils'

definePageMeta({
  middleware: ['auth'],
})

const { params } = useRoute()
const id = typeof params.id === 'string' ? params.id : params.id[0]
const { data: guild, pending } = useFetch<Discord.Guild>(getApiUrl(`/servers?id=${id}`))

useHead({
  title: guild?.value?.name ?? 'サーバー情報',
})
</script>

<template>
  <LayoutContained variant="wide" v-if="!pending && guild">
    <VCard>
      <PageTitle />
      <ServerHeader :server="guild" />
      <ServerFunctions :server="guild" />

      <DevShowCode :code="guild" :prettify="false" />
    </VCard>
  </LayoutContained>
  <LayoutContained variant="compact" v-else-if="!pending && !guild">
    <ServerNotFound />
  </LayoutContained>
  <LayoutCentered variant="wide" v-else>
    <VCard>
      <LoadingCircular>サーバー情報を取得中...</LoadingCircular>
    </VCard>
  </LayoutCentered>
</template>

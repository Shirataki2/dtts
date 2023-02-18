<script setup lang="ts">
import { getApiUrl } from '@/src/utils'
import { Payment } from '@/types'

useHead({
  title: 'アカウント',
})

definePageMeta({
  middleware: ['auth'],
})

const { user } = useUser()

const { data: payments, pending } = useFetch<Payment[]>(getApiUrl('/users/payment'))
</script>

<template>
  <LayoutContained variant="wide">
    <VCard v-if="user">
      <PageTitle>アカウント</PageTitle>
      <AccountHeader :account="user" />

      <AccountPanels>
        <AccountPayment :payments="payments" :pending="pending" />
      </AccountPanels>
    </VCard>
  </LayoutContained>
</template>

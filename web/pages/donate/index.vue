<script lang="ts" setup>
import Stripe from 'stripe'
import { getApiUrl } from '@/src/utils'

definePageMeta({
  middleware: ['auth'],
})

useHead({
  title: '寄付',
})

const { data: prices, pending } = useFetch<Stripe.ApiList<Stripe.Price>>(getApiUrl('/donate/plans'))

const orderedPrices = computed(() => {
  if (prices.value) {
    return prices.value.data.sort((a, b) => {
      const aOrd = a.metadata.order ?? '999'
      const bOrd = b.metadata.order ?? '999'
      return aOrd > bOrd ? 1 : -1
    })
  }
})
</script>

<template>
  <LayoutContained variant="wide">
    <VCard>
      <PageTitle>寄付</PageTitle>
      <VCardText>
        <Paragraph> このサービスを運営するために、運営費の寄付を募集しています。 </Paragraph>
        <Paragraph> 寄付をしていただけると、サービスの運営が円滑に行えるようになります。 </Paragraph>
        <Paragraph> サーバーの有料プランの購入は、こちらのページとは別にございます。 </Paragraph>
        <Paragraph>
          有料プランの購入については、<NuxtLink to="/servers">サーバー一覧</NuxtLink>から所属のサーバーを選択し
          「有料プラン購入」からご購入いただけます。
        </Paragraph>
        <template v-if="!pending && prices?.data">
          <VContainer>
            <VRow>
              <VCol v-for="price in orderedPrices" :key="price.id" cols="12" md="6">
                <PaymentCard :price="price" payment-endpoint="/api/donate/checkout" />
              </VCol>
            </VRow>
          </VContainer>
        </template>
        <LayoutCentered v-else>
          <LoadingCircular> 寄付プランを取得中... </LoadingCircular>
        </LayoutCentered>
      </VCardText>
    </VCard>
  </LayoutContained>
</template>

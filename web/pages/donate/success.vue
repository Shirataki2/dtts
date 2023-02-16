<script lang="ts" setup>
import { getApiUrl } from '@/src/utils'

definePageMeta({
  middleware: ['auth'],
})

const { query } = useRoute()
const isError = ref(false)
const extract = (key: string) => {
  let v = query[key]
  if (v) {
    if (Array.isArray(v)) {
      return v[0]
    }
    return v
  } else {
    isError.value = true
  }
}
const session_id = extract('session_id')
const price_id = extract('price_id')

useHead({
  title: '寄付完了',
})

type Payment = {
  id: string
}

const payment = ref<Payment>()
const pending = ref(true)
const error = ref(false)

onMounted(async () => {
  try {
    const res = await $fetch<Payment>(getApiUrl('/donate/register'), {
      params: {
        session_id,
        price_id,
      },
    })
    payment.value = res
  } catch (e) {
    console.error(e)
    error.value = true
  } finally {
    pending.value = false
  }
})
</script>

<template>
  <LayoutCentered variant="compact">
    <VCard v-if="!isError">
      <PageTitle hide-button>寄付</PageTitle>
      <VCardText>
        <template v-if="payment">
          <Paragraph> 寄付が完了しました。 </Paragraph>
          <Paragraph> ご支援ありがとうございます。 </Paragraph>
          <Paragraph>
            購入情報の確認やキャンセルは
            <NuxtLink to="/account">アカウント</NuxtLink>
            から行えます。
          </Paragraph>
        </template>
        <template v-else-if="pending">
          <LoadingCircular>処理中...</LoadingCircular>
        </template>
        <template v-else-if="error">
          <Paragraph> 処理に失敗しました。 </Paragraph>
        </template>
      </VCardText>
      <VCardActions>
        <VBtn color="primary" block variant="outlined" to="/">トップページへ</VBtn>
      </VCardActions>
    </VCard>
    <VCard v-else>
      <PageTitle>寄付</PageTitle>
      <VCardText>
        <Paragraph> 寄付に失敗しました。 </Paragraph>
      </VCardText>
      <VCardActions>
        <VBtn color="primary" block variant="outlined" to="/">トップページへ</VBtn>
      </VCardActions>
    </VCard>
  </LayoutCentered>
</template>

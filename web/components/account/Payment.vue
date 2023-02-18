<script setup lang="ts">
import { Payment } from '@/types'
import dayjs from 'dayjs'
import utc from 'dayjs/plugin/utc'
import timezone from 'dayjs/plugin/timezone'
dayjs.extend(utc)
dayjs.extend(timezone)

interface Props {
  pending: boolean
  payments: Payment[] | null
}

const props = defineProps<Props>()
const loading = ref(false)

const formatDate = (date: Date) => {
  const tz = dayjs.tz.guess()
  return dayjs.utc(date).tz(tz).format('YYYY/MM/DD HH:mm:ss')
}

const openStripeBillingPortal = () => {
  loading.value = true
  window.location.href = '/api/users/billing_portal'
}
</script>

<template>
  <VExpansionPanel>
    <VExpansionPanelTitle> 決済履歴 </VExpansionPanelTitle>
    <VExpansionPanelText>
      <VBtn
        class="my-4"
        block
        :disabled="loading"
        :loading="loading"
        color="primary"
        variant="outlined"
        @click="openStripeBillingPortal"
      >
        定期決済を管理(外部サイトへ)
      </VBtn>
      <VTable class="my-4">
        <tr>
          <th>日付</th>
          <th>方式</th>
          <th>金額</th>
        </tr>
        <tr v-for="payment in props.payments" :key="payment.subscription_id">
          <td class="date">{{ formatDate(payment.created_at) }}</td>
          <td class="name">{{ payment.name }}</td>
          <td class="price">{{ payment.price.toLocaleString() }}</td>
        </tr>
      </VTable>
    </VExpansionPanelText>
  </VExpansionPanel>
</template>

<style scoped>
.date {
  text-align: center;
}

.name {
  text-align: center;
}

.price {
  text-align: right;
}

.control {
  text-align: center;
}

td {
  padding: 0.5rem;
}
</style>

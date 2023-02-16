<script lang="ts" setup>
import { Stripe } from 'stripe'
interface Props {
  price: Stripe.Price
  paymentEndpoint: string
  priceQuery?: string
}
const props = withDefaults(defineProps<Props>(), {
  priceQuery: 'plan',
})

const paymantName = computed(() => {
  if (props.price.metadata?.name) return props.price.metadata.name
  if (props.price.recurring) {
    const interval = localizeRecurrence(props.price.recurring.interval)
    return `${props.price.recurring.interval_count}${interval}ごと`
  }
  if (props.price.custom_unit_amount) {
    return '任意額(1回)'
  }
  return '不明なプラン'
})

const localizeRecurrence = (interval: Stripe.Price.Recurring.Interval) => {
  switch (interval) {
    case 'day':
      return '日'
    case 'week':
      return '週'
    case 'month':
      return 'ヶ月'
    case 'year':
      return '年'
  }
}

const amountText = computed(() => {
  if (props.price.unit_amount) {
    return props.price.unit_amount.toLocaleString()
  }
  if (props.price.custom_unit_amount?.minimum) {
    return `${props.price.custom_unit_amount.minimum.toLocaleString()}～`
  }
  return '不明な金額'
})

const to = computed(() => {
  if (props.price.active) {
    return `${props.paymentEndpoint}?${props.priceQuery}=${props.price.id}`
  }
  return undefined
})
</script>

<template>
  <VCard variant="outlined" :href="to">
    <VCardTitle class="text-center">
      <VIcon :color="props.price.active ? 'green' : 'red'">
        {{ props.price.active ? 'mdi-check-circle' : 'mdi-alert-circle' }}
      </VIcon>
      {{ paymantName }}
    </VCardTitle>
    <VCardText class="text-center mt-7">
      <span id="currency">￥</span>
      <span id="amount">
        {{ amountText }}
      </span>
    </VCardText>
    <VCardActions>
      <VSpacer />
      <VBtn :disabled="!props.price.active" target="_blank" color="primary" text>
        <VIcon>mdi-credit-card</VIcon>
        <span>寄付する</span>
      </VBtn>
    </VCardActions>
  </VCard>
</template>

<style lang="scss" scoped>
@use 'vuetify/settings';

$curr-size: ();
$curr-size: map-merge(
  (
    'xs': 1.4rem,
    'sm': 1.7rem,
    'md': 2.1rem,
    'lg': 2.5rem,
    'xl': 2.9rem,
  ),
  $curr-size
);

$amount-size: ();
$amount-size: map-merge(
  (
    'xs': 2.4rem,
    'sm': 2.9rem,
    'md': 3.4rem,
    'lg': 3.5rem,
    'xl': 3.6rem,
  ),
  $amount-size
);

@each $bp in map-keys(settings.$display-breakpoints) {
  @media #{map-get(settings.$display-breakpoints, $bp)} {
    #currency {
      font-size: map-get($curr-size, $bp);
      color: goldenrod;
      font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
    }
    #amount {
      font-size: map-get($amount-size, $bp);
      color: goldenrod;
      font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
    }
  }
}
</style>

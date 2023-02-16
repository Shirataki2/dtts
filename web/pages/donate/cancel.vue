<script lang="ts" setup>
useHead({
  title: '寄付',
})
</script>

<template>
  <LayoutCentered variant="compact">
    <VCard>
      <PageTitle hide-button>寄付</PageTitle>
      <VCardText>
        <Paragraph> 寄付をキャンセルしました。 </Paragraph>
      </VCardText>
      <VCardActions>
        <VBtn color="primary" block variant="outlined" to="/">トップページへ</VBtn>
      </VCardActions>
    </VCard>
  </LayoutCentered>
</template>

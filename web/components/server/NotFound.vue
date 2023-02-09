<template>
  <VCard width="100%">
    <VCardTitle class="text-center">
      <strong>サーバーが見つかりませんでした</strong>
    </VCardTitle>
    <VCardText>
      <p class="my-3">リクエストしたIDにマッチするサーバーが見つかりませんでした</p>
      <p class="my-3">サーバーIDが正しいか、またBotがサーバーに導入済みかをご確認ください</p>
      <VBtn block color="primary" to="/servers"> サーバー一覧に戻る </VBtn>
    </VCardText>
  </VCard>
</template>

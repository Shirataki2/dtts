<script setup lang="ts">
import { getApiUrl } from '@/src/utils'
import { ServerPermission, ServerPermissionBody } from '@/types'
import JSONb from 'json-bigint'

useHead({
  title: '権限編集',
})

const { params } = useRoute()
const id = typeof params.id === 'string' ? params.id : params.id[0]

interface MemberPerm {
  is_mod: boolean
}

const perms = ref<ServerPermission[]>([])
const isMod = ref(true)
const { data: memberPerm, pending: pending1 } = useFetch<MemberPerm>(getApiUrl('/servers/perms/check'), {
  params: {
    id: id,
  },
})

watch(memberPerm, val => {
  if (val) {
    isMod.value = val.is_mod
  }
})

const { data: serverPerms, pending: pending2 } = useFetch<ServerPermission[]>(getApiUrl('/servers/perms'), {
  params: {
    id: id,
  },
  parseResponse: res => JSONb.parse(res),
})

watch(serverPerms, val => {
  if (val) {
    perms.value = val
  }
})

const save = async (perms: ServerPermissionBody[]) => {
  console.log(perms)
  return await $fetch(getApiUrl('/servers/perms'), {
    method: 'PATCH',
    body: JSONb.stringify(perms),
    params: {
      id: id,
    },
    headers: {
      'Content-Type': 'application/json',
    },
  })
}
</script>

<template>
  <LayoutContained variant="wide">
    <VCard>
      <PageTitle>権限編集</PageTitle>
      <VCardText>
        <Paragraph>このBotの各サービスごとに内容を変更できるユーザーを制限することができます</Paragraph>
        <Paragraph>各サーバーのユーザーごとの権限を参照して設定できます。</Paragraph>
        <Paragraph>
          (サーバーの所有者、管理者、サーバー管理のいずれかの権限を持っている場合はここで設定した権限に関わらずすべての設定を変更できます。)
        </Paragraph>
        <VAlert border="start" variant="tonal" v-if="!isMod" type="warning" class="my-5">
          以下の設定を変更するには、サーバーの所有者、管理者、サーバー管理のいずれかの権限を持っている必要があります。
        </VAlert>
        <LayoutCentered v-if="pending1 && pending2">
          <VProgressCircular indeterminate />
        </LayoutCentered>
        <PermMatrix v-else :disabled="!isMod || pending1 || pending2" :perms="perms" @save="save" />
      </VCardText>
    </VCard>
  </LayoutContained>
</template>

<script setup lang="ts">
import { getApiUrl } from '@/src/utils'
import { MemberPerm, ServerPermission, ServerPermissionBody } from '@/types'
import JSONb from 'json-bigint'
import { klona } from 'klona'

useHead({
  title: '権限編集',
})

const { params } = useRoute()
const id = typeof params.id === 'string' ? params.id : params.id[0]

const initialPerm = ref<ServerPermission[] | null>(null)
const isDirty = computed(() => {
  return JSONb.stringify(initialPerm.value) !== JSONb.stringify(perms.value)
})
useConfirm(isDirty)

const perms = ref<ServerPermission[]>([])
const isMod = ref(true)
const onLoad = ref(true)
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
    if (onLoad.value) {
      initialPerm.value = klona(val)
      onLoad.value = false
    }
  }
})

const save = async (_perms: ServerPermissionBody[]) => {
  console.log(_perms)
  await $fetch(getApiUrl('/servers/perms'), {
    method: 'PATCH',
    body: JSONb.stringify(_perms),
    params: {
      id: id,
    },
    headers: {
      'Content-Type': 'application/json',
    },
  })
  initialPerm.value = klona(perms.value)
}

const onChange = (val: ServerPermissionBody[]) => {
  perms.value = val.map(v => {
    return {
      guild_id: BigInt(id),
      ...v,
    }
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
        <AppAlert v-if="!isMod" type="warning">
          以下の設定を変更するには、サーバーの所有者、管理者、サーバー管理のいずれかの権限を持っている必要があります。
        </AppAlert>
        <LayoutCentered v-if="pending1 && pending2">
          <LoadingCircular />
        </LayoutCentered>
        <PermMatrix
          v-else
          :disabled="!isMod || pending1 || pending2"
          :is-dirty="isDirty"
          :perms="perms"
          @save="save"
          @change="onChange"
        />
      </VCardText>
    </VCard>
  </LayoutContained>
</template>

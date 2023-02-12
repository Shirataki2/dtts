<script setup lang="ts">
import { Discord } from '@/types'

interface Props {
  server: Discord.Guild
}

const props = defineProps<Props>()

const router = useRouter()

interface Category {
  kind: 'category'
  name: string
}
interface Function {
  kind: 'function'
  name: string
  icon: string
  description: string
  callback: () => void
}
type Item = Category | Function

const items: Item[] = [
  { kind: 'category', name: '読み上げ機能' },
  {
    kind: 'function',
    name: '使用状況',
    icon: 'mdi-chart-areaspline',
    description: 'プランの変更や、読み上げ可能な残りの文字数などの確認を行います。',
    callback: () => {},
  },
  {
    kind: 'function',
    name: '辞書',
    icon: 'mdi-book-open-page-variant',
    description: '読み上げ時に読み方を変えたい単語を登録できます。',
    callback: () => {
      router.push(`/servers/${props.server.id}/dict`)
    },
  },
  {
    kind: 'function',
    name: '読み上げ設定',
    icon: 'mdi-cog',
    description: '読み上げ時の設定を変更できます。',
    callback: () => {},
  },
  { kind: 'category', name: '管理' },
  {
    kind: 'function',
    name: '権限設定',
    icon: 'mdi-account-key',
    description: 'Botのサービスの内容を変更できるユーザーをDiscordの権限をもとに設定します。',
    callback: () => {
      router.push(`/servers/${props.server.id}/permission`)
    },
  },
]

const columnStyle = (col: Item) => {
  if (col.kind === 'category') {
    return {
      cols: 12,
    }
  } else {
    return {
      cols: 12,
      sm: 6,
      md: 4,
    }
  }
}
</script>

<template>
  <VCardText>
    <VContainer>
      <VRow dense>
        <VCol v-for="(item, idx) in items" :key="item.name" v-bind="columnStyle(item)">
          <VCard v-if="item.kind === 'function'" variant="outlined" @click="item.callback" height="100%">
            <VCardTitle>
              <VIcon size="x-large" class="ma-3">{{ item.icon }}</VIcon>
              <span class="text-headline ml-3">{{ item.name }}</span>
            </VCardTitle>
            <VCardText>
              <p class="ma-3">
                {{ item.description }}
              </p>
            </VCardText>
          </VCard>
          <template v-else>
            <VDivider v-if="idx !== 0" class="my-5" />
            <p class="text-center my-3">
              {{ item.name }}
            </p>
          </template>
        </VCol>
      </VRow>
    </VContainer>
  </VCardText>
</template>

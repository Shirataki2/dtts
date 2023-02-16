<script setup lang="ts">
import { NavbarItem } from '@/types'

interface Props {
  item: NavbarItem
}
const props = defineProps<Props>()
const item = props.item
</script>

<template>
  <VListItem
    :to="item.type === 'route' && !item.external ? item.path : undefined"
    :href="item.type === 'route' && item.external ? item.path : undefined"
    :target="item.type === 'route' && item.external ? '_blank' : undefined"
    @click="item.type === 'action' ? item.action() : undefined"
    :title="item.type === 'route' && item.external ? `${item.name} ↗` : item.name"
    active-color="primary"
    :prepend-icon="item.icon"
    :class="item.sub ? 'ml-3' : ''"
    exact
  />
</template>

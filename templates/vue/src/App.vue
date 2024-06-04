<script setup lang="ts">
import NavMenu from './components/NavMenu.vue'
import TailwindIndicator from './components/TailwindIndicator.vue'
import { cn } from '@/lib/utils'
</script>

<template>
  <div class="h-screen overflow-clip">
    <NavMenu />
    <div :class="cn('h-screen overflow-auto border-t bg-background pb-8', 'scrollbar scrollbar-track-transparent scrollbar-thumb-accent scrollbar-thumb-rounded-md')">
      <router-view />
    </div>
  </div>
  <TailwindIndicator />
</template>

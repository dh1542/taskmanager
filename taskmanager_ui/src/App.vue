<template>
  <TaskManagerApp></TaskManagerApp>
</template>


<script setup lang="ts">

import TaskManagerApp from "./apps/TaskManagerApp.vue";




import { onMounted, ref } from "vue";
import { useSystemMetaInfo } from "./composables/useSystemMetaInfo";
import type { MetaInfo } from "@/types/metaInfo";

const serverUrl = 'http://localhost:8000/systemMeta';
let computedMeta = ref<MetaInfo[]>([]);

onMounted(async () => {
  computedMeta.value = await useSystemMetaInfo(serverUrl);

  setInterval(async () => {
    computedMeta.value = await useSystemMetaInfo(serverUrl);
  }, 5000);
})

</script>



<style scoped></style>

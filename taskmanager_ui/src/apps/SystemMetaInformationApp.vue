<template>
    <v-table>
        <thead>
            <tr>
                <th class="text-left">
                    Name
                </th>
                <th class="text-left">
                    Value
                </th>
            </tr>
        </thead>
        <tbody>
            <tr v-for="item in computedMeta" :key="item.name">
                <td>{{ item.name }}</td>
                <td>{{ item.value_s }}</td>
            </tr>
        </tbody>
    </v-table>


</template>

<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useSystemMetaInfo } from "@/composables/useSystemMetaInfo";
import type { MetaInfo } from "@/types/metaInfo";

const serverUrl = 'http://localhost:8000/systemMeta';
let computedMeta = ref<MetaInfo[]>([]);

onMounted(async () => {
    computedMeta.value = await useSystemMetaInfo(serverUrl);

    setInterval(async () => {
        computedMeta.value = await useSystemMetaInfo(serverUrl);
    }, 500);
})


</script>
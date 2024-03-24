<template>
    <v-table>
        <thead>
            <tr>
                <th class="text-left">
                    <h1>Performance</h1>
                </th>
                <th class="text-left"></th>
            </tr>
        </thead>
        <tbody>
            <h3>System</h3>
            <tr v-for="item in cpuSection" :key="item.name">
                <td>{{ item.name }}</td>
                <td>{{ item.value_s }}</td>
            </tr>

            <h3>Memory</h3>
            <tr v-for="item in memorySection" :key="item.name">
                <td>{{ item.name }}</td>
                <td>{{ item.value_s }}</td>
            </tr>

            <h3>Swap</h3>
            <tr v-for="item in swapSection" :key="item.name">
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
let memorySection = ref<MetaInfo[]>([]);
let swapSection = ref<MetaInfo[]>([]);
let cpuSection = ref<MetaInfo[]>([]);

onMounted(async () => {
    computedMeta.value = await useSystemMetaInfo(serverUrl);

    setInterval(async () => {
        computedMeta.value = await useSystemMetaInfo(serverUrl);
        memorySection.value = computedMeta.value.filter((item) => item.name.includes('Memory'));
        swapSection.value = computedMeta.value.filter((item) => item.name.includes('Swap'));
        cpuSection.value = computedMeta.value.filter((item) => !memorySection.value.includes(item) && !swapSection.value.includes(item));
    }, 500);
})
</script>

<style scoped>
.v-table {

    width: 100rem;
}
</style>

<template>
    <div class="main-box">
        <v-card>
            <v-toolbar color="primary">
                <v-toolbar-title>Task Manager</v-toolbar-title>
                <v-spacer></v-spacer>
                <v-btn icon>
                    <v-icon>mdi-magnify</v-icon>
                </v-btn>
                <v-btn icon>
                    <v-icon>mdi-dots-vertical</v-icon>
                </v-btn>
                <template v-slot:extension>
                    <v-tabs v-model="tab" bg-color="primary">
                        <v-tab value="one">Processes</v-tab>
                        <v-tab value="two">Performance</v-tab>

                    </v-tabs>
                </template>
            </v-toolbar>
            <v-card-text>
                <v-window v-model="tab">
                    <v-window-item value="one">
                        <div class="one">
                            Processes will be displayed here
                        </div>

                    </v-window-item>

                    <v-window-item value="two">
                        <div class="one">
                            <SystemMetaInformationApp></SystemMetaInformationApp>
                        </div>

                    </v-window-item>
                </v-window>
            </v-card-text>
        </v-card>
    </div>
</template>

<script setup lang="ts">
import SystemMetaInformationApp from "./SystemMetaInformationApp.vue";

import { ref } from "vue";
const tab = ref<string>('one');

</script>


<style scoped>
.one {
    margin: 0;
    padding: 0;
    width: 100rem;
    height: 100rem;

}

.main-box {

    margin: auto;

    height: 95rem;
    width: 95rem;


}
</style>

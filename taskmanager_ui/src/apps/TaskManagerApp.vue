<template>
    <div class="main-box">
        <v-card>
            <v-toolbar color="primary">
                <v-toolbar-title>Task Manager</v-toolbar-title>
                <v-spacer></v-spacer>
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
                        Processes will be displayed here
                    </v-window-item>

                    <v-window-item value="two">
                        Performance will be displayed here
                    </v-window-item>
                </v-window>
            </v-card-text>
        </v-card>
    </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
const tab = ref<string>('one');

</script>


<style scoped>
.main-box {
    border: 1px solid black;
    margin: auto;
    padding: 2rem 0;
    height: 95rem;
    width: 95rem;


}
</style>

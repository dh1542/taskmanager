<template>
  <h1>HELLO WORLD</h1>
</template>


<script setup lang="ts">
import { useSystemMetaInfo } from "./composables/useSystemMetaInfo";

const serverUrl = 'http://localhost:8000/posts/1';

useSystemMetaInfo(serverUrl)





</script>



<style scoped></style>

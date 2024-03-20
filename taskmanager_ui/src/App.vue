<template>
  <h1>HELLO WORLD</h1>
</template>


<script setup lang="ts">
  const url = "http://localhost:8000/posts/2"
  const response = await fetch(url.toString()).then(
    response => {
      if(!response.ok){
        throw new Error(`HTTP error! status: ${response.status}`);
      }
      return response.text();
    }
  )

  console.log(JSON.parse(response))
  
</script>



<style scoped>

</style>

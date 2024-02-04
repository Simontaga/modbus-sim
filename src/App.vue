<script setup lang="ts">
import SimulatedFan from "./components/SimulatedFan.vue";
import Coil from "./components/Coil.vue";
</script>

<template>
  <div class="container">
    <div class="simulation-container">
      <div v-for="x in 3">
        <SimulatedFan :id="x-1" />
      </div>
    </div>

    <div class="simulation-container">
      <div v-for="x in 10">
        <Coil :id="x-1" />
      </div>
    </div>
    

  </div>
</template>

<style scoped>
</style>


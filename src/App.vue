<script setup lang="ts">
import SimulatedFan from "./components/SimulatedFan.vue";
import Coil from "./components/Coil.vue";
</script>

<template>
  <div class="container">
    <SimulatedFan id="1" />
    <SimulatedFan id="2" />

    <div class="simulation-container">
      <div v-for="x in 10">
        <Coil :id="x" />
      </div>
    </div>
    

  </div>
</template>

<style scoped>
</style>


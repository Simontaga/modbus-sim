<script setup lang="ts">
import { emit, listen } from '@tauri-apps/api/event'
import { ref } from 'vue'

import fan from './fan.svg';
const props = defineProps(['id']);
const value = ref(0);


type DataPoint = {
  x: number;
  y: number;
}

const dataPoints = ref<Array<DataPoint>>([]);

listen<string>(`fan-val-${props.id}`, (event) => {
  let new_value = parseInt(event.payload);
  value.value = new_value;

  dataPoints.value.push({
    x: Date.now(),
    y: new_value
  });

  if (dataPoints.value.length >= 30) {
    dataPoints.value.shift();
  }
});


emit('register-fan', {
  id: parseInt(props.id),
});



</script>

<template>
  <div class="fan-container">
    <img class="fan-spin" :src="fan" alt="fan" width="50px" />
    <div class="fan-info">
      <p>Holding Register: {{ props.id }}</p>
      <p>Current value: {{ value }}</p>
    </div>
  </div>
</template>
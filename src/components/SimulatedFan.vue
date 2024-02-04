<script setup lang="ts">
import { emit, listen } from '@tauri-apps/api/event'
import { computed, ref } from 'vue'
import * as chartConfig from './chartConfig.js'
import VueApexCharts from "vue3-apexcharts";

import fan from './fan.svg';
const props = defineProps(['id']);
const value = ref(0);
const dataPoints = ref([]);

listen(`fan-val-${props.id}`, (event) => {
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

// Compute the chart data
const chartData = computed(() => {
  return [
    {
      name: `Fan ${props.id}`,
      data: [...dataPoints.value]
    }
  ];
})




</script>

<template>
  <div class="fan-container">
    <img class="fan-spin" :src="fan" alt="fan" width="50px" />
    <div class="fan-info">
      <p>Holding Register: {{ props.id }}</p>
      <p>Current value: {{ value }}</p>
    </div>

    <div class="fan-graph">
      <apexchart
      :options="chartConfig.options.chartOptions"
      :series="chartData"
    ></apexchart>
    </div>
  </div>
</template>
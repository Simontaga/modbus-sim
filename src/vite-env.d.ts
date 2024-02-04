/// <reference types="vite/client" />

declare module "*.vue" {
  import type { DefineComponent } from "vue";
  const component: DefineComponent<{}, {}, any>;
  export default component;
}

import ApexCharts from 'apexcharts';

app.config.globalProperties.$apexcharts = ApexCharts;

// Add this when into a TypeScript codebase
declare module '@vue/runtime-core' {
  interface ComponentCustomProperties {
    $apexcharts: typeof ApexCharts;
  }
}
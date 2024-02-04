export const options = {
    chartOptions: {
        chart: {
          id: 'realtime',
          type: 'area',
          width: 250,
          responsive: true,
          parentHeightOffset: 0,
          toolbar: {
            show: false
          },
          zoom: {
            enabled: false
          },
          animations: {
            enabled: true,
            easing: 'linear',
            dynamicAnimation: {
              speed: 1000
            }
          },
        },
        dataLabels: {
          enabled: false
        },
        stroke: {
          curve: 'smooth'
        },
        markers: {
          size: 0
        },
        xaxis: {
            labels: {
                show: false
            },
        },
        yaxis: {
            min: 0,
            max: 100,
        },
        theme: {
            mode: 'dark', 
            palette: 'palette1', 
            monochrome: {
                enabled: false,
                color: '#255aee',
                shadeTo: 'light',
                shadeIntensity: 0.65
            },
        }
      },
};



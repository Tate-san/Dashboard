<script>
    let gaugeValue = 50; // Initial value for the gauge (between 0 and 100)

    import "../app.scss";

    export let deviceName = undefined; // A prop for the device name. 
    let deviceStyle = 0; // A variable to store for the device style - will influence the layout of the gauge.

    console.log(deviceName);

    switch (deviceName) {
      case 'WIFISA10CWT':
      case 'TS0201':
        deviceStyle = 1;
        break;
      case 'ZN231392':
        deviceStyle = 2;
        break;
      default:
        break;
    }

    console.log(deviceStyle);

    function updateGauge(value) {
      gaugeValue = value;
    }
</script>
  
{#if deviceStyle == 1}
  <div class="gauge-container">
    <svg class="gauge-svg" width="334" height="334">
      <!-- Black square background -->
      <rect x="0" y="0" width="240" height="240" fill="#1F1E1F" rx="10" ry="10"></rect>
  
      <!-- Gauge chart -->
      <circle class="gauge-background" cx="120" cy="120" r="80"></circle>
      <circle
        class="gauge-foreground"
        cx="120"
        cy="120"
        r="80"
        :stroke-dasharray="{gaugeValue}, 100"
      ></circle>
      <text x="50%" y="50%" dominant-baseline="middle" text-anchor="middle" class="gauge-text">
        {gaugeValue}%
      </text>
    </svg>
  </div>
    
  <style lang="scss">
  
    .gauge {
      fill: none;

      &-container {
        text-align: center;
        margin-bottom: 20px;
      }
    
      &-svg {
        width: 240px;
        height: 240px;
      }

      &-background {
        stroke: #ccc;
        stroke-width: 20;
      }
      
      &-foreground {
        stroke: #F37748;
        stroke-width: 20;
        transition: stroke-dasharray 0.5s ease-in-out;
      }

      &-text {
        font-size: 35px;
        font-weight: 2400;
        font-family:'Verdana',;
        fill: #fff; 
      }
    }
     
    .buttons {
      text-align: center;
    }
  
    button {
      margin: 10px;
      padding: 10px 20px;
      font-size: 16px;
    }
  </style>
{/if}
  

{#if deviceStyle == 2}
  <div class="gauge-container ">
    <svg class="gauge-svg" width="240" height="240">
      <!-- Black square background -->
      <rect x="0" y="0" width="240" height="240" fill="cyan"></rect>
  
      <!-- Gauge chart -->
      <circle class="gauge-background" cx="120" cy="120" r="80"></circle>
      <circle
        class="gauge-foreground"
        cx="120"
        cy="120"
        r="80"
        :stroke-dasharray="{gaugeValue}, 100"
      ></circle>
      <text x="50%" y="50%" dominant-baseline="middle" text-anchor="middle" class="gauge-text">
        {gaugeValue}%
      </text>
    </svg>
  </div>
  
  <style lang="scss">
  
    .gauge {
      fill: none;

      &-container {
        text-align: center;
        margin-bottom: 20px;
      }
    
      &-svg {
        width: 240px;
        height: 240px;
      }

      &-background {
        stroke: #ccc;
        stroke-width: 20;
      }
    
      &-foreground {
        stroke: #FF5733;
        stroke-width: 20;
        transition: stroke-dasharray 0.5s ease-in-out;
      }
    
      &-text {
        font-size: 24px;
        font-weight: 600;
        font-family:monospace;
        fill: #fff; 
      }
    }
  
  
    .buttons {
      text-align: center;
    }
  
    button {
      margin: 10px;
      padding: 10px 20px;
      font-size: 16px;
    }
  </style>
{/if}
  
<script>
    let gaugeValue = 50; // Initial value for the gauge (between 0 and 100)
  
    function updateGauge(value) {
      gaugeValue = value;
    }
  </script>
  
  <div class="gauge-container">
    <svg class="gauge-svg" width="240" height="240">
      <!-- Black square background -->
      <rect x="0" y="0" width="240" height="240" fill="black"></rect>
  
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
  
  <style>
    .gauge-container {
      text-align: center;
      margin-bottom: 20px;
    }
  
    .gauge-svg {
      width: 240px;
      height: 240px;
    }
  
    .gauge {
      fill: none;
    }
  
    .gauge-background {
      stroke: #ccc;
      stroke-width: 20;
    }
  
    .gauge-foreground {
      stroke: #FF5733;
      stroke-width: 20;
      transition: stroke-dasharray 0.5s ease-in-out;
    }
  
    .gauge-text {
      font-size: 24px;
      font-weight: 600;
      font-family:monospace;
      fill: #fff; 
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

  
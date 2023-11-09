<script>
	// https://caniuse.com/#feat=css-conic-gradients
	// https://css-tricks.com/snippets/css/css-conic-gradient/
	// https://developer.mozilla.org/en-US/docs/Web/CSS/conic-gradient
	
	// https://stackoverflow.com/questions/2465405/svg-angular-gradient
	// https://stackoverflow.com/questions/18206361/svg-multiple-color-on-circle-stroke
	
	// https://bl.ocks.org/mbostock/4163057
	// https://github.com/d3/d3/issues/2427#issuecomment-100759055
	// https://github.com/mnsht/gradient-path
	
	// https://svelte.dev/repl/09711e43a1264ba18945d7db7cab9335?version=3.38.2
  // https://codepen.io/simeydotme/pen/rrOEmO/
	
	import { writable } from 'svelte/store';
	import { tweened, spring } from 'svelte/motion';
	import { backInOut } from 'svelte/easing';
	import { arc as d3arc } from 'd3-shape';
	import { scaleLinear } from 'd3-scale';
	
	let width = 150;
	let height = 150;
	
  // let value = writable(50);
  let value = spring(50, { stiffness: .1 });
  //let value = tweened(50, { easing: backInOut, duration: 750 });
	
	let min = 0;
	let max = 100;
	
	let startAngle = -120;
	let endAngle = 120;
	let innerRadius = 50;
	let outerRadius = 60;
	let cornerRadius = 10;
	
	$: scale = scaleLinear()
		.domain([min, max])
		.range([startAngle, endAngle]);
	
	$: valueAngle = scale($value);

	$: arc = d3arc()
		.innerRadius(innerRadius)
		.outerRadius(outerRadius)
		.startAngle(startAngle * Math.PI / 180)
		.endAngle(valueAngle * Math.PI / 180)
		.cornerRadius(cornerRadius);
	
	$: trackArc = d3arc()
		.innerRadius(innerRadius)
		.outerRadius(outerRadius)
		.startAngle(startAngle * Math.PI / 180)
		.endAngle(endAngle * Math.PI / 180)
		.cornerRadius(cornerRadius);
	
	$: trackArcCentroid = trackArc.centroid()
  // $: console.log(trackArcCentroid)
	
	let trackArcEl
	$: boundingBox = trackArc && trackArcEl ? trackArcEl.getBBox() : {};
  // $: console.log(boundingBox)
	
	$: textArcCenterOffset = {
		x: (outerRadius - (boundingBox.width  / 2)),
    // x: 0,
		y: (outerRadius - (boundingBox.height  / 2)) * -1
	}
  // $: console.log(textArcCenterOffset)
	
		$: textArcBottomOffset = {
		x: (outerRadius - (boundingBox.width  / 2)),
    // x: 0,
		y: (outerRadius - (boundingBox.height)) * -1
	}
  // $: console.log(textArcBottomOffset)
	
	let showTextSvgCenter = true;
	let showTextArcCenter = false;
	let showTextArcBottom = false;
	let showTextArcCentroid = false;
	let showCenterGuide = false;
</script>

<svg {width} {height}>
	<path d={trackArc()} transform="translate({width/2}, {height/2})" class="track" bind:this={trackArcEl} />
	<path d={arc()} transform="translate({width/2}, {height/2})" />

	{#if showTextSvgCenter}
		<text transform="translate({width/2}, {height/2})" dy={16}>
			{Math.round($value)}
		</text>
	{/if}
	
	{#if showTextArcCenter}
		<text x={textArcCenterOffset.x} y={textArcCenterOffset.y} transform="translate({width/2}, {height/2})" dy={16}>
			{Math.round($value)}
		</text>
	{/if}
	
	{#if showTextArcBottom}
		<text x={textArcBottomOffset.x} y={textArcBottomOffset.y} transform="translate({width/2}, {height/2})" dy={0}>
			{Math.round($value)}
		</text>
	{/if}
	
	{#if showTextArcCentroid}
		<text x={trackArcCentroid[0]} y={trackArcCentroid[1]} transform="translate({width/2}, {height/2})" dy={16}>
			{Math.round($value)}
		</text>
	{/if}
	
	{#if showCenterGuide}
		<text transform="translate({width/2}, {height/2})" dy={16}>
			+
		</text>
	{/if}

	<defs>
    <linearGradient id="fillGradient" x1="0%" y1="0%" x2="0%" y2="100%">
      <stop offset="0%"   stop-color="hsl(60, 100%, 50%)"/>
      <stop offset="100%" stop-color="hsl(140, 100%, 50%)"/>
    </linearGradient>
  </defs>
</svg>

<div>
	<label for="">Value:</label>
	<input type="range" {min} {max} bind:value={$value} />
	<input type="number" value={Math.round($value)} on:change={e => $value = Number(e.target.value)} />
</div>

<button on:click={() => $value = min}>Min</button>
<button on:click={() => $value = (min + (max-min) * .25)}>25%</button>
<button on:click={() => $value = (min + (max-min) * .50)}>50%</button>
<button on:click={() => $value = (min + (max-min) * .75)}>75%</button>
<button on:click={() => $value = max}>Max</button>

<div>
	<label for="">Min:</label>
	<input type="range" min={0} max={max} bind:value={min} />
	<input type="number" bind:value={min} />
</div>
	
<div>
	<label for="">Max:</label>
	<input type="range" min={0} max={1000} bind:value={max} />
	<input type="number" bind:value={max} />
</div>



<style>
	svg {
		background-color: hsl(0, 0%, 0%);
		border: 1px solid #ddd;
	}
	path {
		fill: url(#fillGradient);
/* 		fill: red; */
/* 		fill: conic-gradient(gold 40%, #f06 0); */
		
	}
	
	.track {
		stroke: hsla(0, 0%, 100%, .2);
		stroke-width: 1px;
		fill: none;
	}
	
	text {
		fill: white;
		font-size: 2rem;
		text-anchor: middle;
	}
	
	input[type=number] {
    width: 72px;
	}
</style>
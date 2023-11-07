<script>
    import "../app.scss";
    import { Card, Button, Toggle } from 'flowbite-svelte';
    let hCard = false;

	import Gauge from './Gauge.svelte';
</script>

<!-- Device name gets passed to the Gauge component instances as a prop - based on the type of device --> 
<!-- the respective type of layout is rendered. -->  
<Gauge deviceName={"TS0201"}/>
<Gauge deviceName={"WIFISA10CWT"}/>
<!-- Commented this as if there are two styles inside the same file for the same class, the latter takes precedence-->
<!-- <Gauge deviceName={"ZN231392"}/> --> 


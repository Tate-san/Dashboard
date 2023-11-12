<script>
    import "../app.scss";
    import { Card, Button, Toggle } from 'flowbite-svelte';
    let hCard = false;

    import Navbar from "../components/Navbar.svelte";
    import Sidebar  from '../../src/components/Sidebar.svelte';
    import Gauge from "../components/Gauge.svelte";

</script>

<div class="!bg-background w-screen h-screen">
  <Navbar />
  <!-- <Sidebar /> -->
  <Gauge />
</div>

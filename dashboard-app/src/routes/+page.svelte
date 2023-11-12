<script>
    import "../app.scss";
    import { Card, Button, Toggle } from 'flowbite-svelte';
    let hCard = false;

    import Navbar from "../components/Navbar.svelte";
    import Sidebar  from '../../src/components/Sidebar.svelte';
    import Gauge from "../components/Gauge.svelte";

</script>


<style>
    :global(html, body) {
      margin: 0;
      padding: 0;
      height: 100%;
      background-color: #7B7B7B;
    }
</style>

<Navbar />
<!-- <Sidebar /> -->
<Gauge />

<script>
    import "../app.scss";
    import { Card, Button, Toggle } from 'flowbite-svelte';
    let hCard = false;

    import Navbar from "../components/Navbar.svelte";
    import Sidebar  from '../../src/components/Sidebar.svelte';
    import Gauge from "../components/Gauge.svelte";

</script>

<Navbar />
<Sidebar />
<Gauge />

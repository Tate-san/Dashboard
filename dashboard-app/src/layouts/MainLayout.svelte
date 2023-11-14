<script>
    import "../app.scss";
    import { Card, Button, Toggle } from 'flowbite-svelte';
    let hCard = false;

    import Navbar from "../components/Navbar.svelte";
    import Sidebar  from '../../src/components/Sidebar.svelte';

</script>

<div class="!bg-secondary-700 w-screen h-screen">
    <Navbar />
    <slot />
</div>
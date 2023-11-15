<script>
    import "../app.scss";
    import { Card, Button, Toggle } from 'flowbite-svelte';
    let hCard = false;

    import Navbar from "../components/Navbar.svelte";
    import Sidebar  from '../../src/components/Sidebar.svelte';

</script>

<header class="sticky top-0">
    <Navbar />
</header>
<body class="!bg-secondary-700 w-screen h-screen overflow-hidden">
    <slot />
</body>
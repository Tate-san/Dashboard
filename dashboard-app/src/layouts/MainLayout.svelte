<script>
    import "../app.scss";
    import { Card, Button, Toggle } from 'flowbite-svelte';
    let hCard = false;

    import Navbar from "../components/Navbar.svelte";
    import Sidebar  from '../../src/components/Sidebar.svelte';
    import Toasts from "../components/Toasts.svelte";

</script>

<header class="sticky top-0 z-50">
    <Navbar />
</header>
<body class="!bg-secondary-700 w-screen h-screen overflow-hidden relative">
    <Toasts />
    <slot />
</body>
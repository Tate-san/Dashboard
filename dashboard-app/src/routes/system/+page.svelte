<script> 
    import { onMount } from 'svelte';
    import MainLayout from "../../layouts/MainLayout.svelte";
    import { getSystemById } from '../../hooks/systems';
    import { writable } from 'svelte/store';

    let systemId;
    let system = writable();

    onMount(async () => {
        const queryParams = new URLSearchParams(window.location.search);
        systemId = queryParams.get('id');
        getSystemById(systemId)
        .then((data) => {
            system.set(data);
        });
  });
</script>
  
<MainLayout>
    <div class="w-full h-full px-10">
        {system.name}
    </div>
</MainLayout>
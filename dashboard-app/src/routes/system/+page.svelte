<script> 
    import { onMount } from 'svelte';
    import MainLayout from "../../layouts/MainLayout.svelte";
    import { getSystemListByUsers } from '../../hooks/systems';

    let systemId;

    onMount(async () => {
        const queryParams = new URLSearchParams(window.location.search);
        systemId = queryParams.get('id');
  });
</script>
  
<MainLayout>
    <div class="w-full h-full px-10">
        {systemId}
    </div>
</MainLayout>
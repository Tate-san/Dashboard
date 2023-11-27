<script> 
    import { onMount } from 'svelte';
    import MainLayout from "../../layouts/MainLayout.svelte";
    import SystemDetail from "../../components/SystemDetail.svelte";

    let systemId;

    onMount(async () => {
        const queryParams = new URLSearchParams(window.location.search);
        systemId = queryParams.get('id');
  });
</script>
  
<MainLayout>
    <div class="w-full h-full px-10">
        <SystemDetail bind:systemId/>
    </div>
</MainLayout>
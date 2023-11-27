<script> 
    import { onMount } from 'svelte';
    import MainLayout from "../../layouts/MainLayout.svelte";
    import SystemDetail from "../../components/SystemDetail.svelte";
</script>
  
<MainLayout>
    <div class="w-full h-full px-10">
        <SystemDetail />
    </div>
</MainLayout>
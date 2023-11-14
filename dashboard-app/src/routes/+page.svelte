<script> 
  import MainLayout from "../layouts/MainLayout.svelte";
  import Gauge from "../components/Gauge.svelte";
  import {onMount} from "svelte";
  import { Button } from "flowbite-svelte";
  import { getSystemList } from "../hooks/systems";
    import Sidebar from "../components/Sidebar.svelte";

  let systemList = null;

  function fetchUserSystems(){
    getSystemList()
    .then((data) => {
      systemList = data;
    })
    .catch((error) => {
      systemList = null;
    })
  }

  onMount(() => {
    fetchUserSystems();
  });
</script>

<MainLayout>
  <Sidebar>
    
  </Sidebar>
 <!-- <Gauge />-->

  {#if systemList}
  <div class="w-full h-full items-center justify-center flex flex-row flex-wrap text-white gap-x-4 gap-y-2">
    {#each systemList as system}
      <div class="border border-white p-4">
        <p>Name: {system.name}</p> 
        <p>Description: {system.description}</p> 
      </div>
    {/each}
  </div>
  {/if}
</MainLayout>

<script> 
  import MainLayout from "../layouts/MainLayout.svelte";
  import Gauge from "../components/Gauge.svelte";
  import {onMount} from "svelte";
  import { Button, Modal } from "flowbite-svelte";
  import { deleteSystem, getSystemList } from "../hooks/systems";
  import AddSystemForm from "../components/AddSystemForm.svelte";
  import { auth_store } from "../hooks/auth";
  import { writable } from "svelte/store";

  let systemList = writable(null);
  let openAddSystemModal = false;

  function fetchUserSystems(){
    getSystemList()
    .then((data) => {
      systemList.set(data);
    })
    .catch((error) => {
      systemList.set(null);
    })
  }

  onMount(() => {
    fetchUserSystems();
  });

  function onSystemDelete(id){
    deleteSystem(id)
    .then(() => {
      fetchUserSystems();
    });
  }

</script>

<MainLayout>
 <!-- <Gauge />-->
  {#if $auth_store.isLoggedin}
    <Button on:click={() => {openAddSystemModal = true}}>Add system</Button>
  {/if}

  {#if $systemList}
  <div class="w-full h-fit items justify-center flex flex-row flex-wrap text-white gap-x-4 gap-y-2">
    {#each $systemList as system}
      <div class="border border-white p-4 relative">
        <Button class="!p-2 absolute top-2 right-2" on:click={() => {onSystemDelete(system.system_id);}}>
          <i class="fa-solid fa-trash text-white"></i>
        </Button>
    
        <p>Name: {system.name}</p> 
        <p>Description: {system.description}</p> 
      </div>
    {/each}
  </div>
  {/if}
</MainLayout>

<Modal bind:open={openAddSystemModal} size="xs" autoclose={false} class="w-full bg-secondary-800">
  <AddSystemForm bind:clear={openAddSystemModal} onSystemAdded={() => {
    fetchUserSystems(); 
    openAddSystemModal = false;
  }} />
</Modal>

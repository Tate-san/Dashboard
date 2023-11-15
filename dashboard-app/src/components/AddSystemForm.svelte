<script>
    import { Button, Input, Modal } from "flowbite-svelte";
    import { addSystem, system } from '../hooks/systems';
    import { writable } from "svelte/store";

    export let clear = false;
    export let onSystemAdded = () => {};
    let newSystem = system;
    $: clear && clearInput(); 

    function saveSystem()
    {
      addSystem(newSystem)
      .then(() => {
        onSystemAdded();
      });
    }

    function clearInput(){
      newSystem.name = "";
      newSystem.description = "";
    }

</script>

<form class="flex flex-col space-y-6" on:submit={saveSystem}>
  <h3 class="mb-4 text-xl font-medium text-white">Create system</h3>
  <div class="flex w-full">
    <i class="fa-solid fa-microchip m-2 text-white"></i>
    <Input class="h-8 w-full m-0.5" id="name" placeholder="Name" bind:value={newSystem.name}/>
  </div>
  <div class="flex w-full">
    <i class="fa-solid fa-bars-staggered m-2 text-white"></i>
    <Input class="h-8 w-full m-0.5" id="description" placeholder="Description" bind:value={newSystem.description}/>
  </div>
  <Button type="submit" class="w-full1">Save system</Button>
</form>
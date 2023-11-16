<script>
    import { Button, Input, Modal } from "flowbite-svelte";
    import { addSystem, system } from '../hooks/systems';
    import { writable } from "svelte/store";
    import { addToast } from "../hooks/toast";

    export let onSystemEdited = () => {};
    let editedSystem = system; 

    function saveSystem()
    {
      editSystem(editedSystem)
      .then(() => {
        onSystemEdited();
      })
      .catch((e) => {
        addToast({
            message: e.message,
            dismissable: false,
            type: "error"
        });
      });
    }

</script>

<form class="flex flex-col space-y-6" on:submit={saveSystem}>
  <div class="flex w-full">
    <i class="fa-solid fa-microchip m-2 text-white"></i>
    <Input class="h-8 w-full m-0.25 text-xs" id="name" placeholder="Name" bind:value={editedSystem.name}/>
  </div>
  <div class="flex w-full">
    <i class="fa-solid fa-bars-staggered m-2 text-white"></i>
    <Input class="h-8 w-full m-0.25 text-xs" id="description" placeholder="Description" bind:value={editedSystem.description}/>
  </div>
  <Button type="submit" class="w-full1 bg-ming-800 hover:bg-ming-600">Save system</Button>
</form>
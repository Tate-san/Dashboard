<script>
    import { Button, Input, Modal } from "flowbite-svelte";
    import { updateSystem } from '../hooks/systems';
    import { writable } from "svelte/store";
    import { addToast } from "../hooks/toast";

    export let onSystemEdited = () => {};
    export let system = writable({});

    function saveSystem()
    {
      updateSystem(system, system.system_id)
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
    <Input class="h-8 w-full m-0.25 text-xs" id="name" placeholder="Name" bind:value={system.name}/>
  </div>
  <div class="flex w-full">
    <i class="fa-solid fa-bars-staggered m-2 text-white"></i>
    <Input class="h-8 w-full m-0.25 text-xs" id="description" placeholder="Description" bind:value={system.description}/>
  </div>
  <Button type="submit" class="w-full1">Save system</Button>
</form>
<script>
    import { Button, Input, Modal } from "flowbite-svelte";
    import { updateDevice } from '../hooks/devices';
    import { writable } from "svelte/store";
    import { addToast } from "../hooks/toast";
    import { dev } from "$app/environment";

    export let onDeviceEdited = () => {};
    export let device = writable({});

    function saveDevice()
    {
      updateDevice()
      .then(() => {
        onDeviceEdited();
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

<form class="flex flex-col space-y-6" on:submit={saveDevice}>
  <div class="flex w-full">
    <i class="fa-solid fa-microchip m-2 text-white"></i>
    <Input class="h-8 w-full m-0.25 text-xs" id="name" placeholder="Name" bind:value={device.name}/>
  </div>
  <div class="flex w-full">
    <i class="fa-solid fa-bars-staggered m-2 text-white"></i>
    <Input class="h-8 w-full m-0.25 text-xs" id="description" placeholder="Topic" bind:value={device.topic}/>
  </div>
  <Button type="submit" class="w-full1">Save device</Button>
</form>
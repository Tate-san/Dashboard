<script>
    import { Button, Input, Modal } from "flowbite-svelte";
    import { addDevice, device } from '../hooks/devices';
    import { writable } from "svelte/store";
    import { addToast } from "../hooks/toast";

    export let clear = false;
    export let onDeviceAdded = () => {};
    let newDevice = device;
    $: clear && clearInput(); 

    function saveDevice()
    {
      addDevice(newDevice)
      .then(() => {
        onDeviceAdded();
      })
      .catch((e) => {
        addToast({
            message: e.message,
            dismissable: false,
            type: "error"
        });
      });
    }

    function clearInput(){
      newDevice.name = "";
      newDevice.topic = "";
    }

</script>

<form class="flex flex-col space-y-6" on:submit={saveDevice}>
  <div class="flex w-full">
    <i class="fa-solid fa-microchip m-2 text-white"></i>
    <Input class="h-8 w-full m-0.25 text-xs" id="name" placeholder="Name" bind:value={newDevice.name}/>
  </div>
  <div class="flex w-full">
    <i class="fa-solid fa-bars-staggered m-2 text-white"></i>
    <Input class="h-8 w-full m-0.25 text-xs" id="description" placeholder="Topic" bind:value={newDevice.topic}/>
  </div>
  <Button type="submit" class="w-full1 bg-ming-800 hover:bg-ming-600">Save device</Button>
</form>
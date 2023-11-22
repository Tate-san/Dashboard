<script>
    import { Button, Input, Modal, Select } from "flowbite-svelte";
    import { addDevice, device, structureDataTypes, structureValue } from '../hooks/devices';
    import { addToast } from "../hooks/toast";
    import { writable } from "svelte/store";

    export let clear = false;
    export let onDeviceAdded = () => {};
    let newDevice = writable(device);
    $: clear && clearInput(); 

    function saveDevice()
    {
      addDevice($newDevice)
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
      $newDevice.name = "";
      $newDevice.topic = "";
      $newDevice.structure = [];
    }

    function onAddStructureItem(){
      newDevice.set({
        ...$newDevice,
        structure: [...$newDevice.structure, {...structureValue}]
      });
    }

    function onDeleteStructureItem(idx){
      $newDevice.structure.splice(idx, 1);
      newDevice.set({
        ...$newDevice,
      });
    }

</script>

<form class="flex flex-col space-y-6" on:submit={saveDevice}>
  <div class="flex w-full">
    <i class="fa-solid fa-microchip m-2 text-white"></i>
    <Input class="h-8 w-full m-0.25 text-xs" id="name" placeholder="Name" bind:value={$newDevice.name}/>
  </div>
  <div class="flex w-full">
    <i class="fa-solid fa-bars-staggered m-2 text-white"></i>
    <Input class="h-8 w-full m-0.25 text-xs" id="description" placeholder="Topic" bind:value={$newDevice.topic}/>
  </div>
  <Button class="w-4 self-end" on:click={onAddStructureItem}><i class="fa-solid fa-plus"></Button>
  <div class="flex flex-col w-full gap-2">
  {#each $newDevice.structure as structure, i}
    <div class="flex flex-row w-full gap-1 items-center">
      <Input bind:value={structure.real_name} placeholder="Real name" class="h-8 w-full m-0.25 text-xs" />
      <Input bind:value={structure.alias_name} placeholder="Alias name" class="h-8 w-full m-0.25 text-xs" />
      <Select bind:value={structure.data_type} items={structureDataTypes} class="h-8 w-full m-0.25 text-xs" />
      <Button class="w-4 bg-red-500 hover:bg-red-700" on:click={() => onDeleteStructureItem(i)}><i class="fa-solid fa-trash"></i></Button>
    </div>
  {/each}
  </div>
  <Button type="submit" class="w-full1">Save device</Button>
</form>
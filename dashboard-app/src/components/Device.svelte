<script>
    import { Button, Dropdown, DropdownItem } from "flowbite-svelte";
    import { deleteDevice } from "../hooks/devices";
    import { addToast } from "../hooks/toast";
    import { auth_store } from "../hooks/auth";
    import { DotsHorizontalOutline } from "flowbite-svelte-icons";
    import Modal from "./Modal.svelte";
    import EditDeviceForm from "./EditDeviceForm.svelte";

    export let onDeviceChanged = () => {};
    export let device = {};
    let editDevice = {};

    let dropdownOpen = false;
    let openEditModal = false;
    $: isOwner = device.owner_id && (device.owner_id === $auth_store.id);

    function onEditDevice()
    {
      editDevice.device_id = device.device_id;
      editDevice.name = device.name;
      editDevice.topic = device.topic;

      // TODO

      openEditModal = true;
    }

    function onDeviceEdited(){
      openEditModal = false;
      onDeviceChanged();

    }

    function deviceDelete(){
        if(!device) return;

        deleteDevice(device.device_id)
        .then(() => {
          onDeviceChanged();
          openEditModal = false;
        })
        .catch((e) => {
          addToast({
              message: e.message,
              type: "error"
          })
        });
        dropdownOpen = false;
    }
</script>

{#if device}
  <div class="relative flex flex-col max-w-[36rem] gap-1 
      bg-secondary-800 hover:bg-secondary-950 border 
      border-secondary-900 hover:border-ming-400 rounded-lg 
      text-white cursor-pointer">
    <div class="z-10 px-8 py-4" role="button" tabindex="0">
      <h4 class="text-lg font-bold w-[90%] underline">{device.name}</h4>
      <p class="text-xs">{device.topic}</p>
    </div>
    {#if isOwner}
    <DotsHorizontalOutline class="text-white absolute top-0.5 right-1 z-50" />
    <Dropdown bind:open={dropdownOpen} class="bg-secondary-700 rounded-lg text-white border border-secondary-800">
      <DropdownItem on:click={onEditDevice} defaultClass="font-medium py-2 px-4 text-xs hover:bg-secondary-900">Edit</DropdownItem>
      <DropdownItem on:click={deviceDelete} defaultClass="font-medium py-2 px-4 text-xs hover:bg-secondary-900">Delete</DropdownItem>
    </Dropdown>
  {/if}
  </div>
{/if}

<Modal bind:open={openEditModal} size="xs" title="Edit device">
  <EditDeviceForm bind:device={editDevice} onDeviceEdited={onDeviceEdited} />
</Modal>
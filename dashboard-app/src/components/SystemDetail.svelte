<script>
    import { Button, Dropdown, DropdownItem } from "flowbite-svelte";
    import { deleteSystem, removeDeviceFromSystem } from "../hooks/systems";
    import { addToast } from "../hooks/toast";
    import { auth_store } from "../hooks/auth";
    import { DotsHorizontalOutline } from "flowbite-svelte-icons";
    import Modal from "./Modal.svelte";
    import Device from "./Device.svelte";
    import EditSystemForm from "./EditSystemForm.svelte";
    import { getSystemById } from "../hooks/systems";
    import { writable } from "svelte/store";
    import { onMount, afterUpdate, createEventDispatcher } from "svelte";
    import { getDeviceList } from "../hooks/devices";
    import { addDeviceToSystem } from "../hooks/systems";

    let systemId;
    let onSystemChanged = () => {};
    let system = writable({});
    let devices = writable([]);
    let allDevices = writable([]);

    let editSystem = {};
    let dropdownOpen = false;
    let openEditModal = false;
    $: isOwner = $system.owner_id && ($system.owner_id === $auth_store.id);

    function fetchSystem(){
        getSystemById(systemId)
        .then((data) => {
            system.set(data);
        })
        .catch((e) => {
            addToast({
                message: e.message,
                type: "error"
            });
        })
        fetchDevices();
    }

    async function fetchDevices()
    {
        if ($system.devices === undefined) {
        // Fetch the system data if not available
        const data = await getSystemById(systemId).catch((e) => {
            addToast({
                message: e.message,
                type: "error"
            });
        });

        // Set devices to an empty array if data is still not available
        devices.set(data ? data.devices || [] : []);
    } else {
        // $system.devices is already available, use it
        devices.set($system.devices || []);
    }
    }

    function fetchAllDevices()
    {
        getDeviceList()
        .then((data) => {
            allDevices.set(data || []);
        })
    }

    onMount(() => {
        const queryParams = new URLSearchParams(window.location.search);
        systemId = queryParams.get('id');
        fetchSystem(); 
        auth_store.subscribe(() => {
            fetchSystem();
        });
        fetchAllDevices();
    });

    function systemDelete(){
        if(!$system) return;

        deleteSystem($system.system_id)
        .then(() => {
          onSystemChanged();
          openEditModal = false;
        })
        .catch((e) => {
          addToast({
              message: e.message,
              type: "error"
          })
        });
        dropdownOpen = false;
        window.location.href = '/';
    }

    function onEditSystem(){
      editSystem.system_id = $system.system_id;
      editSystem.name = $system.name;
      editSystem.description = $system.description;

      openEditModal = true;
    }

    function onSystemEdited(){
      openEditModal = false;
      onSystemChanged();
      window.location.href = `/system/?id=${systemId}`;
    }

    function addDevice(device_id, system_id)
    {
        addDeviceToSystem(device_id, system_id);
        window.location.href = `/system/?id=${systemId}`;
    }

    function removeDevice(device_id, system_id)
    {
        removeDeviceFromSystem(device_id, system_id);
        window.location.href = `/system/?id=${systemId}`;
    }

</script>

{#if $system}
  <div class="relative flex flex-col max-w-[72rem] gap-1 
      bg-secondary-800 hover:bg-secondary-950 border 
      border-secondary-900 hover:border-primary-400 rounded-lg 
      text-white cursor-pointer">
    <div class="z-10 px-8 py-4" role="button" tabindex="0">
      <p class="text-lg font-bold w-[90%]">Name: {$system.name}</p>
      <p class="text-m">Description: {$system.description}</p>
      <div>
        {#each $devices as device}
        <Device bind:device />
        <Button on:click={() => removeDevice(device.device_id, $system.system_id)}>Remove from system</Button>
        {:else}
        <p>No devices</p>
         {/each}
      </div>
      <div>
        <Button>Add device</Button>
        <Dropdown>
            {#each $allDevices as device}
                <DropdownItem on:click={() => addDevice(device.device_id, $system.system_id)}>{device.name}</DropdownItem>
            {:else}
                <p>No devices</p>
            {/each}
        </Dropdown>
      </div>
    </div>
    {#if isOwner}
      <DotsHorizontalOutline class="text-white absolute top-0.5 right-1 z-50" />
      <Dropdown bind:open={dropdownOpen} class="bg-secondary-700 rounded-lg text-white border border-secondary-800">
        <DropdownItem on:click={onEditSystem} defaultClass="font-medium py-2 px-4 text-xs hover:bg-secondary-900">Edit</DropdownItem>
        <DropdownItem on:click={systemDelete} defaultClass="font-medium py-2 px-4 text-xs hover:bg-secondary-900">Delete</DropdownItem>
      </Dropdown>
    {/if}
  </div>
{/if}
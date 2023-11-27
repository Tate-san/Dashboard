<script>
    import { Button, Dropdown, DropdownItem } from "flowbite-svelte";
    import { deleteSystem } from "../hooks/systems";
    import { addToast } from "../hooks/toast";
    import { auth_store } from "../hooks/auth";
    import { DotsHorizontalOutline } from "flowbite-svelte-icons";
    import Modal from "./Modal.svelte";
    import Device from "./Device.svelte";
    import EditSystemForm from "./EditSystemForm.svelte";
    import { getSystemById } from "../hooks/systems";
    import { writable } from "svelte/store";
    import { onMount, afterUpdate } from "svelte";

    let systemId;
    let onSystemChanged = () => {};
    let system = writable({});

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
    }

    onMount(() => {
        const queryParams = new URLSearchParams(window.location.search);
        systemId = queryParams.get('id');
        fetchSystem(); 
        auth_store.subscribe(() => {
            fetchSystem();
        });
        console.log(`systemId = ${systemId}`);
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

</script>

{#if $system}
  <div class="relative flex flex-col max-w-[36rem] gap-1 
      bg-secondary-800 hover:bg-secondary-950 border 
      border-secondary-900 hover:border-primary-400 rounded-lg 
      text-white cursor-pointer">
    <div class="z-10 px-8 py-4" role="button" tabindex="0">
      <h4 class="text-lg font-bold w-[90%] underline">{$system.name}</h4>
      <p class="text-xs"></p>
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

<Modal bind:open={openEditModal} size="xs" title="Edit system">
  <EditSystemForm bind:system={editSystem} onSystemEdited={onSystemEdited} />
</Modal>
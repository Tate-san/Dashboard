<script>
    import { Button, Dropdown, DropdownItem } from "flowbite-svelte";
    import { deleteSystem } from "../hooks/systems";
    import { addToast } from "../hooks/toast";
    import { auth_store } from "../hooks/auth";
    import { DotsHorizontalOutline } from "flowbite-svelte-icons";
    import Modal from "./Modal.svelte";
    import EditSystemForm from "./EditSystemForm.svelte";

    export let onSystemChanged = () => {};
    export let system = {};
    let editSystem = {};
    let dropdownOpen = false;
    let openEditModal = false;
    $: isOwner = system.owner_id && (system.owner_id === $auth_store.id);

    function systemDelete(){
        if(!system) return;

        deleteSystem(system.system_id)
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
    }

    function onEditSystem(){
      editSystem.system_id = system.system_id;
      editSystem.name = system.name;
      editSystem.description = system.description;

      openEditModal = true;
    }

    function onSystemEdited(){
      openEditModal = false;
      onSystemChanged();

    }

    function systemOpen(systemId){
      window.location.href = `/system/?id=${systemId}`;
    }

</script>

{#if system}
  <div class="relative flex flex-col max-w-[36rem] gap-1 
      bg-secondary-800 hover:bg-secondary-950 border 
      border-secondary-900 hover:border-primary-400 rounded-lg 
      text-white cursor-pointer">
    <div class="z-10 px-8 py-4" on:click={() => systemOpen(system.system_id)} on:keypress={() => systemOpen(system.system_id)} role="button" tabindex="0">
      <h4 class="text-lg font-bold w-[90%] underline">{system.name}</h4>
      <p class="text-xs">{system.description}</p>
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
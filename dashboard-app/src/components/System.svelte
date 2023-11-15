<script>
    import { Button, Dropdown, DropdownItem } from "flowbite-svelte";
    import { deleteSystem } from "../hooks/systems";
    import { addToast } from "../hooks/toast";
    import { auth_store } from "../hooks/auth";
    import { DotsHorizontalOutline } from "flowbite-svelte-icons";

    export let onSystemDelete = () => {};
    export let system = {};
    let dropdownOpen = false;
    $: isOwner = system.owner_id && (system.owner_id === $auth_store.id) || 0;

    function systemDelete(){
        if(!system) return;

        deleteSystem(system.system_id)
        .then(() => {
          onSystemDelete();
          addToast({
              message: "System successfully deleted",
              type: "success"
          });
        })
        .catch((e) => {
          addToast({
              message: e.message,
              type: "error"
          })
        });
        dropdownOpen = false;
    }

    function systemOpen(){
      console.log("Open system " + system.system_id);
    }

</script>

{#if system}
  <div class="relative flex flex-col max-w-[36rem] gap-1 
      bg-secondary-800 hover:bg-primary-950 border 
      border-secondary-900 hover:border-primary-700 rounded-lg 
      text-white cursor-pointer">
    <div class="z-10 px-8 py-4" on:click={systemOpen} on:keypress={systemOpen} role="button" tabindex="0">
      <h4 class="text-xl font-bold w-[90%]">{system.name}</h4>
      <p>{system.description}</p>
    </div>
    {#if isOwner}
      <DotsHorizontalOutline class="text-white absolute top-2 right-2 z-50" />
      <Dropdown bind:open={dropdownOpen} class="bg-secondary-700 rounded-lg text-white border border-secondary-800">
        <DropdownItem defaultClass="font-medium py-2 px-4 text-sm hover:bg-secondary-900">Edit</DropdownItem>
        <DropdownItem on:click={systemDelete} defaultClass="font-medium py-2 px-4 text-sm hover:bg-secondary-900">Delete</DropdownItem>
      </Dropdown>
    {/if}
  </div>
{/if}

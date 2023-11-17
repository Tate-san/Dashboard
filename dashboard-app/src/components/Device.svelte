<script>
    import { Button, Dropdown, DropdownItem } from "flowbite-svelte";
    import { addToast } from "../hooks/toast";
    import { auth_store } from "../hooks/auth";
    import { DotsHorizontalOutline } from "flowbite-svelte-icons";
    import Modal from "./Modal.svelte";

    export let onDeviceChanged = () => {};
    export let device = {};
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
  </div>
{/if}

<Modal size="xs" title="Edit device">

</Modal>
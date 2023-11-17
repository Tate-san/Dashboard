<script>
    import { Accordion, AccordionItem, Button, List} from "flowbite-svelte";
    import { getDeviceList } from "../hooks/devices";
    import { onMount } from "svelte";
    import { writable } from "svelte/store";
    import { addToast } from "../hooks/toast";
    import { auth_store } from "../hooks/auth";
    import AddDeviceForm from "./AddDeviceForm.svelte"
    import Device from "./Device.svelte"
    import Modal from "./Modal.svelte";
    import { goto } from '$app/navigation';

    let deviceList = writable([]);
    let openAddDeviceModal = false;

    function fetchDeviceList(){
        if(!$auth_store.isLoggedin) {
            window.location.href = "/"; 
            return;
        }

        getDeviceList()
        .then((data) => {
            deviceList.set(data);
            console.log(data)
        })
        .catch((e) => {
            addToast({
                message: e.message,
                type: "error"
            });
        })
    }

    onMount(() => {
        fetchDeviceList(); 
        auth_store.subscribe(() => {
            fetchDeviceList();
        });
    });

</script>

{#if $auth_store.isLoggedin}
    <Button on:click={() => {openAddDeviceModal = true}} class="bg-ming-600 my-2 mx-2 hover:bg-ming-800">
        <i class="fa-solid fa-plus mr-3"></i>    
        New device
    </Button>
{/if}

{#each $deviceList as device}
    <Device bind:device onDeviceChanged={fetchDeviceList} />
{/each}

<Modal title="Create device" bind:open={openAddDeviceModal} size="xs">
    <AddDeviceForm bind:clear={openAddDeviceModal} onDeviceAdded={() => {
      openAddDeviceModal = false;
      fetchDeviceList();
    }} />
</Modal>
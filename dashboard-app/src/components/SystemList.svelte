<script>
    import { Accordion, AccordionItem, Button} from "flowbite-svelte";
    import { getSystemListByUsers } from "../hooks/systems";
    import { onMount } from "svelte";
    import { writable } from "svelte/store";
    import { addToast } from "../hooks/toast";
    import { auth_store } from "../hooks/auth";
    import AddSystemForm from "./AddSystemForm.svelte";
    import System from "./System.svelte";
    import Modal from "./Modal.svelte";

    let userList = writable([]);
    let openAddSystemModal = false;

    function fetchSystemList(){
        getSystemListByUsers()
        .then((data) => {
            userList.set(data);
        })
        .catch((e) => {
            addToast({
                message: e.message,
                type: "error"
            });
        })
    }

    onMount(() => {
        fetchSystemList(); 
    });

</script>

{#if $auth_store.isLoggedin}
    <Button on:click={() => {openAddSystemModal = true}} class="my-2 mx-2">
        <i class="fa-solid fa-plus mr-3"></i>    
        New system
    </Button>
{/if}

<Accordion multiple activeClass="bg-black-950 text-secondary-50" inactiveClass="bg-secondary-800 hover:bg-black-950 text-secondary-50" class="bg-black-700 rounded-t-xl border-transparent ">
    {#each $userList as user}
        {#if user.systems.length}
            <AccordionItem borderSharedClass="border-secondary-900">
                <span slot="header" class="text-sm font-regular font-sans">{user.username}'s systems</span>
                    <div class="flex flex-row gap-8 flex-wrap w-full">
                        {#each user.systems as system}
                            <System bind:system onSystemChanged={fetchSystemList} />
                        {/each}
                    </div>
            </AccordionItem>
        {/if} 
    {/each}
</Accordion>

<Modal title="Create system" bind:open={openAddSystemModal} size="xs">
  <AddSystemForm bind:clear={openAddSystemModal} onSystemAdded={() => {
    openAddSystemModal = false;
    fetchSystemList();
  }} />
</Modal>
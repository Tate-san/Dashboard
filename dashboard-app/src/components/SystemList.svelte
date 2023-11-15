<script>
    import { Accordion, AccordionItem, Button, Modal } from "flowbite-svelte";
    import { deleteSystem, getSystemListByUsers } from "../hooks/systems";
    import { onMount } from "svelte";
    import { writable } from "svelte/store";
    import { addToast } from "../hooks/toast";
    import { auth_store } from "../hooks/auth";
    import AddSystemForm from "./AddSystemForm.svelte";
    import System from "./System.svelte";

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

    function onSystemDelete(id){
        deleteSystem(id)
        .then(() => {
            fetchSystemList();

            addToast({
                message: "System successfully deleted",
                type: "success"
            });
        });
    }

    onMount(() => {
        fetchSystemList(); 
    });

</script>

{#if $auth_store.isLoggedin}
    <Button on:click={() => {openAddSystemModal = true}}>Add system</Button>
{/if}

<Accordion multiple activeClass="bg-primary-900 text-secondary-50" inactiveClass="bg-secondary-800 hover:bg-primary-800 text-secondary-50">
    {#each $userList as user}
        {#if user.systems.length}
            <AccordionItem borderSharedClass="border-secondary-500">
                <span slot="header">{user.username}'s systems</span>
                <div class="flex flex-row gap-4 flex-wrap w-full">
                    {#each user.systems as system}
                        <System bind:system onSystemDelete={fetchSystemList} />
                    {/each}
                </div>
            </AccordionItem>
        {/if} 
    {/each}
</Accordion>

<Modal bind:open={openAddSystemModal} size="xs" autoclose={false} class="w-full bg-secondary-800">
  <AddSystemForm bind:clear={openAddSystemModal} onSystemAdded={() => {
    openAddSystemModal = false;
    fetchSystemList();
  }} />
</Modal>
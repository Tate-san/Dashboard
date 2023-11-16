<script>
    import { Button, Modal } from "flowbite-svelte";
    import { CloseOutline } from "flowbite-svelte-icons";


    export let open = false;
    export let autoclose = false;
    export let outsideclose = false;
    export let dismissable = true;
    export let size = 'md';
    export let title = '';
    export let placement = 'center';
    export let dialogClass = 'fixed top-0 left-0 right-0 h-modal md:inset-0 md:h-full z-50 w-full p-4 flex';
    export let defaultClass = 'relative flex flex-col mx-auto';
    export let backdropClass = 'fixed inset-0 z-40 bg-gray-900 bg-opacity-50 dark:bg-opacity-80';

    function closeModal(){
        open = false;
    }
</script>

<Modal bind:open 
        bind:size 
        bind:autoclose
        bind:outsideclose
        bind:title
        bind:placement
        bind:dialogClass
        bind:defaultClass
        bind:backdropClass
        dismissable={false} 
        class="w-full bg-secondary-900">
    {#if dismissable}
        <Button class="!p-2 z-10 absolute top-2 right-2 bg-transparent hover:bg-secondary-50 hover:text-secondary-700" on:click={closeModal}>
            <CloseOutline class="w-3 h-3" />
        </Button>
    {/if}
    <slot />
</Modal>
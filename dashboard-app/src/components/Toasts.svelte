<script>
    import { Toast } from "flowbite-svelte";
    import { toasts } from "../hooks/toast";
</script>

<div class="flex flex-col gap-2 items-center w-full max-h-0 z-50 absolute top-2 left-0">
    {#each $toasts as toast}
        <Toast class="bg-secondary-600 border border-secondary-800" bind:open={toast.open} dismissable={toast.dismissable}>
            <div class="flex flex-row items-center gap-2 w-full">
                <i class="fa-solid {toast.icon} {toast.color} text-lg"></i>
                <p class="text-secondary-50">{toast.message}</p> 
            </div>
        </Toast>
    {/each}
</div>
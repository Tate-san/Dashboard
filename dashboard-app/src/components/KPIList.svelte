<script>
    import KPI from "./KPI.svelte"
    import Modal from "./Modal.svelte";
    import AddKPIForm from "./AddKPIForm.svelte";

    import { Button } from 'flowbite-svelte';

    let openAddKPIModal = false;
</script>


<Button on:click={() => {openAddKPIModal = true}} class="my-2 mx-2">
    <i class="fa-solid fa-plus mr-3"></i>    
    New KPI
</Button>

<KPI/>

<Modal title="Create KPI" bind:open={openAddKPIModal} size="xs">
    <AddKPIForm bind:clear={openAddKPIModal} onKPIAdded={() => {
      openAddKPIModal = false;
    //   fetchSystemList();
    }} />
  </Modal>
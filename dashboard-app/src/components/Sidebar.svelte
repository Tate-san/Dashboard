<script>
    import { Sidebar, SidebarGroup, SidebarItem, SidebarWrapper, SidebarDropdownItem, SidebarDropdownWrapper, Modal, Input, Button } from 'flowbite-svelte';
    import { ChartPieSolid, GridSolid, ShoppingCartSolid, MailBoxSolid, UserSolid, ArrowRightToBracketSolid, FileEditSolid, PlusOutline } from 'flowbite-svelte-icons';
    import { addSystem, system } from '../hooks/systems';

    let open = false;

    function openModal()
    {
      open = !open;
    }

    let newSystem = system;

    function saveSystem()
    {
      addSystem(newSystem);
    }
</script>
  
<Sidebar color="green">
  <SidebarWrapper>
    <SidebarGroup>
      <SidebarItem label="Dashboard">
        <svelte:fragment slot="icon">
          <ChartPieSolid class="w-5 h-5 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white" />
        </svelte:fragment>
      </SidebarItem>
      <SidebarDropdownWrapper label="Your Systems">
        <svelte:fragment slot="icon">
          <ShoppingCartSolid class="w-5 h-5 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white" />
        </svelte:fragment>
        <SidebarDropdownItem label="System 1" />
        <SidebarDropdownItem label="System 2" />
        <SidebarDropdownItem label="System 3" />
      </SidebarDropdownWrapper>
      
      <SidebarDropdownWrapper label="All Systems">
          <svelte:fragment slot="icon">
            <ShoppingCartSolid class="w-5 h-5 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white" />
          </svelte:fragment>
          <SidebarDropdownItem label="System 1" />
          <SidebarDropdownItem label="System 2" />
          <SidebarDropdownItem label="System 3" />
          <SidebarDropdownItem label="System 4" />
          <SidebarDropdownItem label="System 5" />
      </SidebarDropdownWrapper>
      <SidebarItem label="Add new system" on:click={openModal}>
        <svelte:fragment slot="icon">
          <PlusOutline class="w-5 h-5 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white" />
        </svelte:fragment>
      </SidebarItem>
      <SidebarItem label="Settings">
          <svelte:fragment slot="icon">
            <ChartPieSolid class="w-5 h-5 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white" />
          </svelte:fragment>
        </SidebarItem>
    </SidebarGroup>
  </SidebarWrapper>
</Sidebar>

<Modal bind:open={open} size="xs" autoclose={true} class="w-full bg-secondary-800">
  <div class="flex w-full">
    <i class="fa-solid fa-microchip m-2 text-white"></i>
    <Input class="h-8 w-full m-0.5" id="name" placeholder="Name" bind:value={newSystem.name}/>
  </div>
  <div class="flex w-full">
    <i class="fa-solid fa-bars-staggered m-2 text-white"></i>
    <Input class="h-8 w-full m-0.5" id="description" placeholder="Description" bind:value={newSystem.description}/>
  </div>
  <Button class="w-full1" on:click={saveSystem}>Save system</Button>
</Modal>
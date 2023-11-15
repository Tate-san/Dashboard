<script>
    import { Sidebar, SidebarGroup, SidebarItem, SidebarWrapper, SidebarDropdownItem, SidebarDropdownWrapper, Modal, Input, Button } from 'flowbite-svelte';
    import { ChartPieSolid, GridSolid, ShoppingCartSolid, MailBoxSolid, UserSolid, ArrowRightToBracketSolid, FileEditSolid, PlusOutline } from 'flowbite-svelte-icons';

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


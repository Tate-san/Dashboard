<script>
  import { Navbar, NavBrand, NavLi, NavUl,
            Dropdown, DropdownItem, DropdownDivider, 
            Input, Label, Button, Alert, A } from 'flowbite-svelte';
  import { ChevronDownOutline } from 'flowbite-svelte-icons';
  import { page } from '$app/stores';
  import { InfoCircleSolid } from 'flowbite-svelte-icons';
  import RegisterForm from './RegisterForm.svelte';
  import { auth_store, logout } from '../hooks/auth';
  import LoginForm from './LoginForm.svelte';
  import Modal from "./Modal.svelte";

  $: activeUrl = $page.url.pathname;

  let openRegisterModal = false;

</script>

<div class="!bg-secondary-950 w-screen max-h-[4rem] h-[4rem] px-10 py-2 flex flex-row justify-between"> 

  <div class="flex flex-row gap-x-4">

    <NavBrand class="text-white text-2xl hover:text-primary-400" href="/">
      <i class="fa-solid fa-chevron-left mr-2"></i>
    </NavBrand>

    <NavBrand class="text-white text-2xl hover:text-primary-400" href="/">
      <i class="fa-solid fa-chevron-right mr-2"></i>
    </NavBrand>

    <NavBrand class="text-white text-xl hover:text-primary-400" href="/">
      <i class="fa-solid fa-house mr-2"></i>
    </NavBrand>
  
  </div>

  <div class="flex flex-row items-center gap-x-6">
    <Input id="search-navbar" class="max-h-[2rem] max-w-[15rem]" placeholder="&#xF002; Search..." />

    <NavUl {activeUrl} ulClass="text-md">
    {#if $auth_store.isLoggedin}
      <NavLi class="cursor-pointer text-white whitespace-nowrap">
        {$auth_store.username}<ChevronDownOutline class="w-3 h-3 ml-2 inline" />
      </NavLi>
        
      <Dropdown class="w-44 z-20 bg-secondary-800 text-secondary-200 border border-secondary-600 rounded-lg">
        <DropdownItem class="hover:text-primary-400 hover:bg-secondary-800" href="/">Dashboard</DropdownItem>
        <DropdownItem class="hover:text-primary-400 hover:bg-secondary-800" href="/docs/components/navbar">Settings</DropdownItem>
        <DropdownItem class="hover:text-primary-400 hover:bg-secondary-800" href="/">Earnings</DropdownItem>
        <DropdownDivider />
        <DropdownItem class="hover:text-primary-400 hover:bg-secondary-800" on:click={logout} >Logout</DropdownItem>
      </Dropdown>
    {:else}
      <NavLi class="cursor-pointer text-white !hover:text-orange-600 whitespace-nowrap">
        Not logged in<ChevronDownOutline class="w-3 h-3 ml-2 inline" />
      </NavLi>
      <Dropdown class="w-96 z-20 bg-secondary-800 text-secondary-200 border border-secondary-600 rounded-lg">
        <LoginForm onRegister={() => {openRegisterModal = true}}/>
      </Dropdown>
    {/if}
    </NavUl>
  </div>
</div>

<Modal bind:open={openRegisterModal} size="xs" title="Register">
  <RegisterForm onUserRegistered={() => {openRegisterModal = false}}/>
</Modal>

<style lang="scss">
  :global(#search-navbar) {
    font-family: 'Helvetica', FontAwesome, sans-serif;
    font-style: normal;
    font-weight: normal;
    text-decoration: inherit;
  }
</style>
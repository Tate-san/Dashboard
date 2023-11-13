<script>
  import { auth_store, login, logout }  from "../hooks/auth";
  import { Navbar, NavBrand, NavLi, NavUl, NavHamburger, Dropdown, DropdownItem, DropdownDivider, Input, Label, Button } from 'flowbite-svelte';
  import { ChevronDownOutline } from 'flowbite-svelte-icons';
  import { page } from '$app/stores';
  import { Alert } from 'flowbite-svelte';
  import { InfoCircleSolid } from 'flowbite-svelte-icons';

  $: activeUrl = $page.url.pathname;

  var loginForm = {
    username: "",
    password: ""
  }

  var isLoginInvalid = false;

  function loginUser(){

    login(loginForm.username, loginForm.password).then((a) => {
      loginForm = {
        username: "",
        password: "",
      }
      isLoginInvalid = false;
    }).catch((e) => {
      isLoginInvalid = true;
    })
  }

</script>

<div class="!bg-navbar-bg w-screen max-h-[4rem] h-[4rem] px-10 py-2 flex flex-row justify-between"> 

  <div class="flex flex-row gap-x-4">

    <NavBrand class="text-white text-2xl hover:text-orange-600" href="/">
      <i class="fa-solid fa-chevron-left mr-2"></i>
    </NavBrand>

    <NavBrand class="text-white text-2xl hover:text-orange-600" href="/">
      <i class="fa-solid fa-chevron-right mr-2"></i>
    </NavBrand>

    <NavBrand class="text-white text-xl hover:text-orange-600" href="/">
      <i class="fa-solid fa-house mr-2"></i>
    </NavBrand>"
  
  </div>

  <div class="flex flex-row items-center gap-x-6">
    <Input id="search-navbar" class="max-h-[2rem] max-w-[15rem]" placeholder="&#xF002; Search..." />

    <NavUl {activeUrl} ulClass="text-md">
    {#if $auth_store.isLogged}
      <NavLi class="cursor-pointer text-white !hover:text-orange-600 whitespace-nowrap">
        {$auth_store.username}<ChevronDownOutline class="w-3 h-3 ml-2 inline" />
      </NavLi>
        
      <Dropdown class="w-44 z-20">
        <DropdownItem href="/">Dashboard</DropdownItem>
        <DropdownItem href="/docs/components/navbar">Settings</DropdownItem>
        <DropdownItem href="/">Earnings</DropdownItem>
        <DropdownDivider />
        <DropdownItem on:click={logout} >Logout</DropdownItem>
      </Dropdown>
    {:else}
      <NavLi class="cursor-pointer text-white !hover:text-orange-600 whitespace-nowrap">
        Not logged in<ChevronDownOutline class="w-3 h-3 ml-2 inline" />
      </NavLi>
      <Dropdown class="w-80 z-20 bg-slate-600" on:show={() => isLoginInvalid = false}>
          <div class="w-full h-fit flex flex-col p-2 gap-2">
            {#if isLoginInvalid}
            <Alert color="red" dismissable>
              <InfoCircleSolid slot="icon" class="w-4 h-4" />
              <span class="font-small">Invalid username or password!</span>
            </Alert>
            {/if}
            <Label for="username">Username:</Label>
            <Input class="h-8" id="username" bind:value={loginForm.username}/>
            <Label for="password">Password:</Label>
            <Input class="h-8" id="password" type="password" bind:value={loginForm.password}/>
            <Button class="w-fit self-center" on:click={loginUser}>Log in</Button>
          </div>
      </Dropdown>
    {/if}
    </NavUl>
  </div>
</div>

<style lang="scss">
  :global(#search-navbar) {
    font-family: 'Helvetica', FontAwesome, sans-serif;
    font-style: normal;
    font-weight: normal;
    text-decoration: inherit;
  }
</style>
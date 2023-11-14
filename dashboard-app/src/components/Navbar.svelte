<script>
  import { auth_store, login, logout, register }  from "../hooks/auth";
  import { Navbar, NavBrand, NavLi, NavUl,
            Dropdown, DropdownItem, DropdownDivider, 
            Input, Label, Button, Alert, A,
            Modal} from 'flowbite-svelte';
  import { ChevronDownOutline } from 'flowbite-svelte-icons';
  import { page } from '$app/stores';
  import { InfoCircleSolid } from 'flowbite-svelte-icons';

  $: activeUrl = $page.url.pathname;

  let loginForm = {
    username: "",
    password: ""
  }

  let registerForm = {
    username: "",
    password: ""
  }

  let isLoginInvalid = false;

  let registerState = {
    modal: false,
    isInvalid: false,
    errorMessage: ""
  };

  function onLoginUser(){
    isLoginInvalid = false;

    login(loginForm.username, loginForm.password).then((a) => {
      loginForm = {
        username: "",
        password: "",
      }
    }).catch((e) => {
      isLoginInvalid = true;
    })
  }

  function onRegisterUser(){
    registerState.isInvalid = false;

    register(registerForm.username, registerForm.password).then(() => {
        registerForm = {
          username: "",
          password: ""
        };
        registerState.modal = false;
    }).catch((e) => {
        registerState.isInvalid = true;
        registerState.errorMessage = e.response.data.message;
    })
  }

</script>

<div class="!bg-secondary-950 w-screen max-h-[4rem] h-[4rem] px-10 py-2 flex flex-row justify-between fixed top-0"> 

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
      <Dropdown class="w-96 z-20 bg-secondary-800 text-secondary-200 border border-secondary-600 rounded-lg" on:show={() => isLoginInvalid = false}>
          <div class="w-full h-fit flex flex-col px-12 py-4 gap-2 items-center">
            {#if isLoginInvalid}
            <Alert color="red">
              <InfoCircleSolid slot="icon" class="w-4 h-4" />
              <span class="font-small">Invalid username or password!</span>
            </Alert>
            {/if}
            <div class="flex w-full">
              <i class="fa-solid fa-user m-2 text-white"></i>
              <Input class="h-8 w-full m-0.5" id="username" placeholder="Username" bind:value={loginForm.username}/>
            </div>
            
            <div class="flex w-full">
              <i class="fa-solid fa-lock m-2 text-white"></i>
              <Input class="h-8 w-full m-0.5" id="password" type="password" placeholder="Password" bind:value={loginForm.password}/>
            </div>
            
            <Button class="w-fit" on:click={onLoginUser}>Sign in</Button>
            <A class="font-medium hover:underline text-primary-400" on:click={() => {registerState.modal = true; registerState.isInvalid = false;}}>Create new account</A>
          </div>
      </Dropdown>
    {/if}
    </NavUl>
  </div>
</div>

<Modal bind:open={registerState.modal} on:show={() => registerState.isInvalid = false} size="xs" autoclose={false} class="w-full bg-secondary-800">
  <div class="flex flex-col space-y-6">
    <h3 class="mb-4 text-xl font-medium text-white">Register</h3>
    {#if registerState.isInvalid}
      <Alert color="red">
        <InfoCircleSolid slot="icon" class="w-4 h-4" />
        <span class="font-small">{registerState.errorMessage}</span>
      </Alert>
    {/if}

    <div class="flex w-full">
      <i class="fa-solid fa-user m-2 text-white"></i>
      <Input class="h-8 w-full m-0.5" id="username" placeholder="Username" bind:value={registerForm.username}/>
    </div>

    <div class="flex w-full">
      <i class="fa-solid fa-lock m-2 text-white"></i>
      <Input class="h-8 w-full m-0.5" id="password" type="password" placeholder="Password" bind:value={registerForm.password}/>
    </div>

    <Button class="w-full1" on:click={onRegisterUser}>Create account</Button>
  </div>
</Modal>

<style lang="scss">
  :global(#search-navbar) {
    font-family: 'Helvetica', FontAwesome, sans-serif;
    font-style: normal;
    font-weight: normal;
    text-decoration: inherit;
  }
</style>
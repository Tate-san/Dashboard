<script>
    import { Navbar, NavBrand, NavLi, NavUl, NavHamburger, Dropdown, DropdownItem, DropdownDivider, Input } from 'flowbite-svelte';
    import { ChevronDownOutline } from 'flowbite-svelte-icons';
    import { page } from '$app/stores';
    $: activeUrl = $page.url.pathname;
</script>

<div class="navbar" color="">
  <Navbar color="!bg-blue-500"> 
    <NavBrand href="/">
      <i class="fa-solid fa-house mr-2"></i>
      <span class="self-center whitespace-nowrap text-xl font-semibold dark:text-white">Dashboard</span>
    </NavBrand>
  
    <NavBrand href="/">
      <i class="fa-solid fa-right-long mr-2"></i>
      <span class="self-center whitespace-nowrap text-r font-regular dark:text-white">Forward</span>
    </NavBrand>
  
    <NavBrand href="/">
      <i class="fa-solid fa-left-long mr-2"></i>
      <span class="self-center whitespace-nowrap text-r font-regular dark:text-white">Back</span>
    </NavBrand>
  
    <div>
      <Input id="search-navbar" class="pl-10" placeholder="Search..." />
    </div>
    
    <NavUl {activeUrl}>
      <NavLi href="/">Home</NavLi>
      <NavLi class="cursor-pointer">
        Regular user<ChevronDownOutline class="w-3 h-3 ml-2 text-primary-800 dark:text-white inline" />
      </NavLi>
      <Dropdown class="w-44 z-20">
        <DropdownItem href="/">Dashboard</DropdownItem>
        <DropdownItem href="/docs/components/navbar">Settings</DropdownItem>
        <DropdownItem href="/">Earnings</DropdownItem>
        <DropdownDivider />
        <DropdownItem href="/">Sign out</DropdownItem>
      </Dropdown>
    </NavUl>
  </Navbar>
</div>

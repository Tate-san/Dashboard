<script>
    import { Navbar, NavBrand, NavLi, NavUl, NavHamburger, Dropdown, DropdownItem, DropdownDivider } from 'flowbite-svelte';
    import { ChevronDownOutline } from 'flowbite-svelte-icons';
    import { page } from '$app/stores';
    $: activeUrl = $page.url.pathname;
</script>
  
<Navbar color="blue">
  <NavBrand href="/">
    <span class="self-center whitespace-nowrap text-xl font-semibold dark:text-white">IIS</span>
  </NavBrand>
  <NavHamburger />
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
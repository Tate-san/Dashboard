<script>
    import { Navbar, NavBrand, NavLi, NavUl, NavHamburger, Dropdown, DropdownItem, DropdownDivider, Input } from 'flowbite-svelte';
    import { ChevronDownOutline } from 'flowbite-svelte-icons';
    import { page } from '$app/stores';
    $: activeUrl = $page.url.pathname;
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
      <NavLi class="cursor-pointer text-white !hover:text-orange-600 whitespace-nowrap">
        Regular user<ChevronDownOutline class="w-3 h-3 ml-2 inline" />
      </NavLi>
      <Dropdown class="w-44 z-20">
        <DropdownItem href="/">Dashboard</DropdownItem>
        <DropdownItem href="/docs/components/navbar">Settings</DropdownItem>
        <DropdownItem href="/">Earnings</DropdownItem>
        <DropdownDivider />
        <DropdownItem href="/">Sign out</DropdownItem>
      </Dropdown>
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
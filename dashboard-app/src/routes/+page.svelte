<script> 
  import MainLayout from "../layouts/MainLayout.svelte";
  import Gauge from "../components/Gauge.svelte";
  import {onMount} from "svelte";
  import { Button } from "flowbite-svelte";
  import {login, logout, auth_store} from "../hooks/auth";

  function onLogin(){
    login("test", "test");
    console.log("Login");
  }

  function onLogout(){
    logout();
    console.log("Logout");
  }

</script>

<MainLayout>
  <Gauge />
</MainLayout>

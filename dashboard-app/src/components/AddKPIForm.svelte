<script>
    import { Button, Input, Modal } from "flowbite-svelte";
    import { addSystem, system } from '../hooks/systems';
    import { writable } from "svelte/store";
    import { addToast } from "../hooks/toast";

    export let clear = false;
    export let onKPIAdded = () => {};
    let newSystem = system;
    $: clear && clearInput(); 

    function saveSystem()
    {
      addSystem(newSystem)
      .then(() => {
        onKPIAdded();
      })
      .catch((e) => {
        addToast({
            message: e.message,
            dismissable: false,
            type: "error"
        });
      });
    }

    function clearInput(){
      newSystem.name = "";
      newSystem.description = "";
    }

</script>

<form class="flex flex-col space-y-4">
  <div class="flex w-full">
    <Input class="h-8 w-full m-0.25 text-xs" id="param" placeholder="Parameter"/>
  </div>
  <div class="flex w-full">
    <Input class="h-8 w-full m-0.25 text-xs" id="limit" placeholder="Limitation"/>
  </div>
  <div class="flex w-full">
    <Input class="h-8 w-full m-0.25 text-xs" id="value" placeholder="Value"/>
  </div>
  <Button type="submit" class="w-full1">Save KPI</Button>
</form>
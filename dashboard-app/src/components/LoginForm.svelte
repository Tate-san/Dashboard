<script>
    import { InfoCircleSolid } from "flowbite-svelte-icons";
    import { login } from "../hooks/auth";
    import { A, Alert, Button, Input } from "flowbite-svelte";
    import { addToast } from "../hooks/toast";

    export let onRegister = () => {}
    export let clear = false;
    $: clear && clearInput();

    let loginForm = {
        username: "",
        password: ""
    }

    function onLoginUser(){

        login(loginForm.username, loginForm.password).then((a) => {
            clearInput();

            addToast({
                message: "Logged in",
                type: "success"
            });
        }).catch((e) => {

            addToast({
                message: e.message,
                dismissable: false,
                type: "error"
            });
        })
    }

    function clearInput(){
        loginForm.username = "";
        loginForm.password = "";
    }
</script>


<form class="w-full h-fit flex flex-col px-12 py-4 gap-2 items-center" on:submit={onLoginUser}>
<div class="flex w-full">
    <i class="fa-solid fa-user m-2 text-white"></i>
    <Input class="h-8 w-full m-0.5" id="username" placeholder="Username" bind:value={loginForm.username}/>
</div>

<div class="flex w-full">
    <i class="fa-solid fa-lock m-2 text-white"></i>
    <Input class="h-8 w-full m-0.5" id="password" type="password" placeholder="Password" bind:value={loginForm.password}/>
</div>

<Button type="submit" class="w-fit">Sign in</Button>
<A on:click={onRegister} class="font-medium hover:underline text-primary-400">Create new account</A>
</form>
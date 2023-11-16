<script>
    import { Alert, Button, Input } from "flowbite-svelte";
    import { InfoCircleSolid } from "flowbite-svelte-icons";
    import { auth_store, login, logout, register }  from "../hooks/auth";
    import { addToast } from "../hooks/toast";

    export let onUserRegistered = () => {}; 
    export let clear = false;
    $: clear && clearInput();

    let registerForm = {
        username: "",
        password: ""
    }

    let registerState = {
        isInvalid: false,
        errorMessage: ""
    };

    function onRegisterUser(){
        registerState.isInvalid = false;

        register(registerForm.username, registerForm.password).then(() => {
            onUserRegistered();

            addToast({
                message: "Successfully registered",
                type: "success"
            });
            
        }).catch((e) => {
            registerState.isInvalid = true;
            registerState.errorMessage = e.response.data.message;
        })
    }

    function clearInput(){
        registerState.isInvalid = false;
        registerForm.username = "";
        registerForm.password = "";
    }
</script>

<form class="flex flex-col space-y-6" on:submit={onRegisterUser}>
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

    <Button type="submit" class="w-full1">Create account</Button>
</form>
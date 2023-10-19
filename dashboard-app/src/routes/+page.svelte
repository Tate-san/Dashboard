<script>
    import "../app.scss";
    import { Card, Button, Toggle } from 'flowbite-svelte';
    let hCard = false;
</script>

<div class="w-full h-screen flex flex-col justify-center items-center">
    <h1 class="text-4xl font-bold mb-4">Hello World</h1>
    <Card img="https://i.cbc.ca/1.5359228.1577206958!/fileImage/httpImage/image.jpg_gen/derivatives/16x9_940/smudge-the-viral-cat.jpg" href="/" horizontal reverse={hCard} class="mb-4">
        <h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white">Noteworthy technology acquisitions 2021</h5>
        <p class="mb-3 font-normal text-gray-700 dark:text-gray-400 leading-tight">Here are the biggest enterprise technology acquisitions of 2021 so far, in reverse chronological order.</p>
    </Card>
    <Toggle bind:checked={hCard} class="italic dark:text-gray-500">Reverse</Toggle>
</div>


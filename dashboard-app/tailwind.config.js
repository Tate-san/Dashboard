/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}', './node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}'],
  plugins: [require('flowbite/plugin')],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        // flowbite-svelte
        background: '#7B7B7B',
        navbar: {
          bg: '#1F1E1F',
        },
        primary: {
          '50': '#f4f6f7',
          '100': '#e3e7ea',
          '200': '#cad2d7',
          '300': '#a5b2bb',
          '400': '#798a97',
          '500': '#5e6f7c',
          '600': '#515e69',
          '700': '#424b54',
          '800': '#3e454c',
          '900': '#373c42',
          '950': '#22262a',
        },
        secondary: {
          '50': '#f7f8f8',
          '100': '#eef0f0',
          '200': '#d9dede',
          '300': '#c4cbca',
          '400': '#929e9c',
          '500': '#748381',
          '600': '#5e6b69',
          '700': '#4d5755',
          '800': '#424a49',
          '900': '#3a403f',
          '950': '#262b2b',
        }
      }
    }
  },
  plugins: []
};
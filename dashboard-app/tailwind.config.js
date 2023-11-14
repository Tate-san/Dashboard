/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}', './node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}'],
  plugins: [require('flowbite/plugin')],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        background: '#7B7B7B',
        navbar: {
          bg: '#1F1E1F',
        },
        primary: {
              '50': '#eff9ff',
    '100': '#dbf2fe',
    '200': '#bfe9fe',
    '300': '#93dcfd',
    '400': '#60c6fa',
    '500': '#3baaf6',
    '600': '#1a88ea',
    '700': '#1d76d8',
    '800': '#1e5faf',
    '900': '#1e518a',
    '950': '#173254',
        },
        secondary: {
          '50': '#f7f8f8',
          '100': '#eeeef0',
          '200': '#d9dade',
          '300': '#b8bac1',
          '400': '#91949f',
          '500': '#737784',
          '600': '#5d606c',
          '700': '#4c4e58',
          '800': '#41424b',
          '900': '#303136',
          '950': '#26272b',
        }
      }
    }
  },
  plugins: []
};
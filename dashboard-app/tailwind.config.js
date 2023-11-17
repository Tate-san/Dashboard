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
          '50': '#eefdfd',
          '100': '#d3f8fa',
          '200': '#acf0f5',
          '300': '#73e3ed',
          '400': '#32ccde',
          '500': '#16afc4',
          '600': '#158da5',
          '700': '#187084',
          '800': '#1d5c6d',
          '900': '#1c4d5d',
          '950': '#0d333f',
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
        },
        'green': {
          '50': '#f0fdf4',
          '100': '#dbfde7',
          '200': '#baf8d0',
          '300': '#84f1ab',
          '400': '#57e389',
          '500': '#1fc85c',
          '600': '#14a548',
          '700': '#13823c',
          '800': '#156633',
          '900': '#13542c',
          '950': '#042f16',
      },
      'black': {
        '50': '#f6f6f6',
        '100': '#e7e7e7',
        '200': '#d1d1d1',
        '300': '#b0b0b0',
        '400': '#888888',
        '500': '#6d6d6d',
        '600': '#5d5d5d',
        '700': '#4f4f4f',
        '800': '#454545',
        '900': '#3d3d3d',
        '950': '#000000',
      },

      }
    }
  },
  plugins: []
};
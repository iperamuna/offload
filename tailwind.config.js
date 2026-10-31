/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        brand: {
          50: '#eef6ff',
          100: '#d9eaff',
          200: '#bbd9fe',
          300: '#8dc1fd',
          400: '#589ef9',
          500: '#327ef4',
          600: '#1f61e7',
          700: '#174bd3',
          800: '#193eaa',
          900: '#193686',
        },
        slate: {
          850: '#151f32',
          950: '#070b14',
        }
      },
      fontFamily: {
        sans: ['-apple-system', 'BlinkMacSystemFont', '"SF Pro Display"', '"Segoe UI"', 'Roboto', 'sans-serif'],
      }
    },
  },
  plugins: [],
}


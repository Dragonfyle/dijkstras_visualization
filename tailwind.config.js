/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.rs", "./index.html"],
  theme: {
    extend: {
      gridTemplateColumns: {
        '50': 'repeat(50, minmax(0, 1fr))',
      },
      gridTemplateRows: {
        '50': 'repeat(50, minmax(0, 1fr))',
      },
      height: {
        '800': '800px',
        '600': '600px',
        '400': '400px',
      },
      width: {
        '800': '800px',
        '600': '600px',
        '400': '400px',
      },
      gap: {
        '0.25': '0.0625rem',
      },
    },
  },
  plugins: [],
}


/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{html,js,ts,tsx}"],
  theme: {
    extend: {
      colors: {
        primary: "#e76f51",
        secondary: "#f4a261",
        thirdinary: "#e9c46a",
        supporting: "#2a9d8f",
        dark_supporting: "#264653",
      },
    },
  },
  plugins: [],
};

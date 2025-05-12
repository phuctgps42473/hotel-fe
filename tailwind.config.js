/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "jit",
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  plugins: [
    require("daisyui"), require("@tailwindcss/typography")
  ],
  theme: {
    // fontFamily: {
    // sans: ["Open Sans", "Noto Color Emoji"],
    // display: ["Comfortaa", "Noto Color Emoji"],
    // mono: ["Fira Mono", "Noto Color Emoji"],
    // },
  },
  daisyui: {
    themes: [
      "light",
      "dark",
    ],
  },
};
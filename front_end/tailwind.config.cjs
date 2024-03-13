module.exports = {
  content: ['./src/**/*.{svelte,js,ts}'],
  plugins: [require("@tailwindcss/typography"), require('@tailwindcss/forms'), require('daisyui')],
  daisyui: {
    themes: [
      {
        mytheme: {
          "primary": "#42a909",
          "secondary": "#bad532",
          "accent": "#179246",
          "neutral": "#3d4451",
          "base-100": "#ffffff",
        },
      },
      "light"],
  },
  container: {
    center: true
  },
  variants: {
    extend: {
      display: ["group-hover"],
    },
  },
}

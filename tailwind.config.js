/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: [
    // include all rust, html and css files in the src directory
    "./src/**/*.{rs,html,css}",
    // include all html files in the output (dist) directory
    "./dist/**/*.html",
  ],
  theme: {
    extend: {},
  },
  plugins: [require("daisyui")],

  daisyui: {
    themes: [
      {
        Light: {
          ...require("daisyui/src/theming/themes")["light"],
          primary: "#fab937",
          secondary: "#77c2f7",
        },
        Dark: {
          ...require("daisyui/src/theming/themes")["dark"],
          primary: "#fab937",
          secondary: "#77c2f7",
          "base-100": "#181818",
          "base-200": "#242424",
          "base-300": "#303030",
        },
      },
    ],
  },
}
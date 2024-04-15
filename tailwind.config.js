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
  plugins: [
    require("daisyui"),
    require("@catppuccin/tailwindcss"),
  ],
	safelist: [
		{
			pattern: /bg-.+/
		},
		'mocha',
		'macchiato',
		'frappe',
		'latte'
	],

  daisyui: {
    themes: [
      {
        "Latte": {
         "primary": "#fe640b", // peach
         "secondary": "#04a5e5", // sky
         "accent": "#8839ef", // mauve
         "base-100": "#bcc0cc", // surface1
         "base-200": "#ccd0da", // surface0
         "base-300": "#eff1f5", // base
         "base-content": "#4c4f69", // text
         "neutral": "#dce0e8", // crust
         "info": "#209fb5", // saphire
         "success": "#40a02b", // green
         "warning": "#df8e1d", // yellow
         "error": "#d20f39", // red
        },
        "Frappe": {
         "primary": "#ef9f76", // peach
         "secondary": "#99d1db", // sky
         "accent": "#ca9ee6", // mauve
         "base-100": "#51576d", // surface1
         "base-200": "#414559", // surface0
         "base-300": "#303446", // base
         "base-content": "#c6d0f5", // text
         "neutral": "#232634", // crust
         "info": "#85c1dc", // saphire
         "success": "#a6d189", // green
         "warning": "#e5c890", // yellow
         "error": "#e78284", // red
        },
        "Macchiato": {
         "color-scheme": "dark",
         "primary": "#f5a97f", // peach
         "secondary": "#91d7e3", // sky
         "accent": "#c6a0f6", // mauve
         "base-100": "#494d64", // surface1
         "base-200": "#363a4f", // surface0
         "base-300": "#24273a", // base
         "base-content": "#cad3f5", // text
         "neutral": "#181926", // crust
         "info": "#7dc4e4", // saphire
         "success": "#a6da95", // green
         "warning": "#eed49f", // yellow
         "error": "#ed8796", // red
        },
        "Mocha": {
         "color-scheme": "dark",
         "primary": "#fab387", // peach
         "secondary": "#89dceb", // sky
         "accent": "#cba6f7", // mauve
         "base-100": "#1e1e2e", // surface1
         "base-200": "#313244", // surface0
         "base-300": "#45475a", // base
         "base-content": "#cdd6f4", // text
         "neutral": "#11111b", // crust
         "info": "#74c7ec", // saphire
         "success": "#a6e3a1", // green
         "warning": "#f9e2af", // yellow
         "error": "#f38ba8", // red
        },
      },
    ],
  },
}
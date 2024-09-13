import type { Config } from "tailwindcss"
import plugin from "tailwindcss/plugin"

const config: Config = {
  darkMode: ["class"],
  content: [
    "./pages/**/*.{js,ts,jsx,tsx,mdx}",
    "./components/**/*.{js,ts,jsx,tsx,mdx}",
    "./app/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    extend: {
      fontFamily: {
        montserrat: ["Montserrat", "sans-serif"],
      },
      colors: {
        text: {
          50: "#f2eef7",
          100: "#e4ddee",
          200: "#cabade",
          300: "#af98cd",
          400: "#9475bd",
          500: "#7a53ac",
          600: "#61428a",
          700: "#493267",
          800: "#312145",
          900: "#181122",
          950: "#0c0811",
        },
        background: {
          50: "#f2edf7",
          100: "#e6dbf0",
          200: "#ccb8e0",
          300: "#b294d1",
          400: "#9970c2",
          500: "#7f4db3",
          600: "#663d8f",
          700: "#4c2e6b",
          800: "#331f47",
          900: "#190f24",
          950: "#0d0812",
        },
        primary: {
          50: "#f1edf8",
          100: "#e3dbf0",
          200: "#c7b6e2",
          300: "#ab92d3",
          400: "#8f6dc5",
          500: "#7349b6",
          600: "#5c3a92",
          700: "#452c6d",
          800: "#2e1d49",
          900: "#170f24",
          950: "#0b0712",
        },
        secondary: {
          50: "#f1ecf8",
          100: "#e2daf1",
          200: "#c6b4e4",
          300: "#a98fd6",
          400: "#8c69c9",
          500: "#7044bb",
          600: "#593696",
          700: "#432970",
          800: "#2d1b4b",
          900: "#160e25",
          950: "#0b0713",
        },
        accent: {
          50: "#f1ecf9",
          100: "#e2d8f3",
          200: "#c6b2e6",
          300: "#a98bda",
          400: "#8d65cd",
          500: "#703ec1",
          600: "#5a329a",
          700: "#432574",
          800: "#2d194d",
          900: "#160c27",
          950: "#0b0613",
        },
      },
      borderRadius: {
        lg: "var(--radius)",
        md: "calc(var(--radius) - 2px)",
        sm: "calc(var(--radius) - 4px)",
      },
      keyframes: {
        "accordion-down": {
          from: { height: "0" },
          to: { height: "var(--radix-accordion-content-height)" },
        },
        "accordion-up": {
          from: { height: "var(--radix-accordion-content-height)" },
          to: { height: "0" },
        },
      },
      animation: {
        "accordion-down": "accordion-down 0.2s ease-out",
        "accordion-up": "accordion-up 0.2s ease-out",
      },
    },
  },
  plugins: [
    require("tailwindcss-animate"),
    require("@tailwindcss/aspect-ratio"),
    require("@tailwindcss/typography"),
    require("@tailwindcss/forms"),
    plugin(({ addBase }) => {
      addBase({
        "@font-face": {
          fontFamily: "Montserrat",
          fontWeight: "300 400 500 600 700",
          src: `
            url("./fonts/Montserrat-Light.ttf") format("truetype"),
            url("./fonts/Montserrat-Regular.ttf") format("truetype"),
            url("./fonts/Montserrat-Medium.ttf") format("truetype"),
            url("./fonts/Montserrat-SemiBold.ttf") format("truetype"),
            url("./fonts/Montserrat-Bold.ttf") format("truetype")
          `,
        },
      })
    }),
  ],
  variants: {
    extend: {
      backdropFilter: ["responsive"],
    },
  },
}
export default config

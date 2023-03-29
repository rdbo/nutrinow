/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
      "./index.html",
      "./src/**/*.{js,ts,jsx,tsx,vue}"
  ],
  theme: {
    extend: {
      dropShadow: {
        "bold-sm": [
            "-1px -1px 0 #606060",
            "1px -1px 0 #606060",
            "-1px 1px 0 #606060",
            "1px 1px 0 #606060"
        ]
      },
      colors: {
          "primary": {
              "100": "#99ff55",
              "200": "#55ff1a"
          },
          "secondary": {
              "100": "#fff3e0",
              "200": "#f44336"
          }
      },
      fontFamily: {
          body: ["Arvo"]
      },
      spacing: {
        '100': '34rem',
        '128': '42rem',
      }
    },
  },
  plugins: [],
}

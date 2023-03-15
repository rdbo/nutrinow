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
            "0 0 8px rgba(0.1, 0.1, 0.1, 0.5)",
            "0 0 8px rgba(0.1, 0.1, 0.1, 0.5)",
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
        }
    },
  },
  plugins: [],
}

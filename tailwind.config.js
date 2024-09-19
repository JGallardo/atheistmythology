/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
    content: ["./src/**/*.rs", "./public/**/*.html"]
  },
  theme: {
    extend: {},
  },
  plugins: [],
}

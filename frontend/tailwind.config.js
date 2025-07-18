module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      boxShadow: {
        'neon-purple': '0 0 20px #a78bfa, 0 0 40px #7c3aed',
        'neon-green': '0 0 20px #6ee7b7, 0 0 40px #10b981',
        'soft-glow': '0 4px 32px 0 rgba(124, 58, 237, 0.25)',
      },
      dropShadow: {
        'neon': '0 0 10px #7c3aed',
        'soft': '0 2px 8px rgba(0,0,0,0.15)',
      },
    },
  },
  plugins: [],
} 
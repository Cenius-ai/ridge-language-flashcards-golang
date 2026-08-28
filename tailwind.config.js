/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: 'class',
  content: ['./templates/**/*.html'],
  theme: {
    extend: {
      colors: {
        surface: 'var(--color-surface)',
        'surface-raised': 'var(--color-surface-raised)',
        'surface-muted': 'var(--color-surface-muted)',
        border: 'var(--color-border)',
        fg: 'var(--color-fg)',
        'fg-muted': 'var(--color-fg-muted)',
        accent: 'var(--color-accent)',
        'accent-fg': 'var(--color-accent-fg)',
        'accent-hover': 'var(--color-accent-hover)',
        'accent-muted': 'var(--color-accent-muted)',
      },
      fontFamily: {
        sans: ['Onest', 'system-ui', 'sans-serif'],
        display: ['Onest', 'system-ui', 'sans-serif'],
      },
      borderRadius: {
        card: '0.5rem',
      },
    },
  },
  plugins: [],
};

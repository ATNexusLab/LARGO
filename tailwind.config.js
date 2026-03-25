module.exports = {
  content: ["./src/**/*.{js,ts,jsx,tsx}", "./docs/**/*.md", "./docs/ui/examples/**/*.md"],
  theme: {
    extend: {
      colors: {
        bg: 'var(--color-bg)',
        surface: 'var(--color-surface)',
        primary: 'var(--color-primary)',
        secondary: 'var(--color-secondary)',
        text: 'var(--color-text)',
        muted: 'var(--color-muted)'
      },
      fontFamily: {
        sans: ['Azonix', 'Inter', 'system-ui']
      },
      spacing: {
        '1': '4px',
        '2': '8px',
        '3': '12px',
        '4': '16px',
        '5': '24px',
        '6': '32px'
      }
    }
  },
  plugins: []
};

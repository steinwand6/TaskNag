/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        'task-bg': '#f8fafc',
        'task-card': '#ffffff',
        'task-border': '#e2e8f0',
        'priority-low': '#10b981',
        'priority-medium': '#f59e0b',
        'priority-high': '#ef4444',
        'priority-critical': '#dc2626',
        'status-inbox': '#64748b',
        'status-todo': '#3b82f6',
        'status-progress': '#8b5cf6',
        'status-done': '#10b981',
      },
      animation: {
        'fade-in': 'fadeIn 0.2s ease-in-out',
        'slide-up': 'slideUp 0.3s ease-out',
      },
      keyframes: {
        fadeIn: {
          '0%': { opacity: '0' },
          '100%': { opacity: '1' },
        },
        slideUp: {
          '0%': { transform: 'translateY(10px)', opacity: '0' },
          '100%': { transform: 'translateY(0)', opacity: '1' },
        },
      },
    },
  },
  plugins: [],
}
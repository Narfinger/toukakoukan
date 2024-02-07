import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import { VitePWA } from 'vite-plugin-pwa'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [VitePWA({
    registerType: 'autoUpdate', workbox: {
      globPatterns: ['**/*.{js,css,html,ico,png,svg}'],
    }
  }),
  svelte()
  ]
})

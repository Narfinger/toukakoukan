import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import { VitePWA } from 'vite-plugin-pwa'


// https://vitejs.dev/config/
export default defineConfig({
  plugins: [VitePWA({
    devOptions: {
      enabled: true
    },
    workbox: {
      globPatterns: ['**/*.{js,css,html,ico,png,svg}'],

      runtimeCaching: [{
        urlPattern: "http://localhost:.*\\/api\\/.*",
        handler: "NetworkFirst",
        options: {
          cacheName: "api-cache",
          cacheableResponse: {
            statuses: [0, 200]
          }
        }
      }]
    },
    registerType: 'autoUpdate',
  }),
  svelte()
  ]
})

import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import { VitePWA } from 'vite-plugin-pwa'

// https://vitejs.dev/config/
export default defineConfig({
  server: {
    proxy: {
      '/api': {
        target: 'http://localhost:3000/',
        changeOrigin: true,
        secure: false,
      },
      '/auth': {
        target: 'http://localhost:3000/',
        changeOrigin: true,
        secure: false,
      },
      '/user_creation': {
        target: 'http://localhost:3000/',
        changeOrigin: true,
        secure: false,
      },
    },
  },
  plugins: [VitePWA({
    manifest: {
      theme_color: '#ffffff',
      icons: [
        {
          src: "/512.png",
          sizes: "512x512",
          type: "image/png",
          purpose: "any maskable"
        }
      ]
    },
    devOptions: {
      enabled: true
    },
    workbox: {
      globPatterns: ['**/*.{js,css,html,ico,png,svg}'],
      runtimeCaching: [
        {
          urlPattern: ({ url, sameOrigin }) => url.pathname.match(/^\/api/),
          handler: 'StaleWhileRevalidate',
          method: 'GET',
          options: {
            cacheName: 'api-cache',
            expiration: {
              maxEntries: 20,
              maxAgeSeconds: 60 * 60 * 24 * 7, // <== 365 days
            },
            cacheableResponse: {
              statuses: [0, 200],
            },
          },
        }, {
          handler: 'NetworkOnly',
          urlPattern: ({ url, sameOrigin }) => url.pathname.match(/^\/api/),
          method: 'POST',
          options: {
            backgroundSync: {
              name: 'api-queue',
              options: {
                maxRetentionTime: 24 * 60
              }
            }
          }
        },
      ]
    }
  }),
  svelte()
  ]
})

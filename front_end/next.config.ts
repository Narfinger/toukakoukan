import type { NextConfig } from "next";
module.exports = {
  async rewrites() {
    return {
      fallback: [
        // These rewrites are checked after both pages/public files
        // and dynamic routes are checked
        {
          source: '/:path*',
          destination: `http://localhost:3001/:path*`,
        },
      ],
    }
  },
}
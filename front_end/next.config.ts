import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  /* config options here */
};

module.exports = {
  async rewrites() {
    return [
      {
        source: '/api',
        destination: 'http://localhost:3000/api',
      },
    ]
  },
}

export default nextConfig;

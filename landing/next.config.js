/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,

  // Enable static export for GitHub Pages
  output: 'export',

  // Base path for GitHub Pages (repo name)
  basePath: '/English-Liminal-',

  // Asset prefix for GitHub Pages
  assetPrefix: '/English-Liminal-/',

  // Disable image optimization for static export
  images: {
    unoptimized: true,
  },

  // Trailing slash for better compatibility
  trailingSlash: true,
}

module.exports = nextConfig

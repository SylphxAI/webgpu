import { defineConfig } from 'vitepress'

export default defineConfig({
  title: '@sylphx/webgpu',
  description: 'WebGPU for Node.js & Bun - Modern, lightweight alternative to Dawn',

  ignoreDeadLinks: true,

  head: [
    ['link', { rel: 'icon', href: '/favicon.ico' }],
    ['meta', { name: 'theme-color', content: '#646cff' }],
    ['meta', { property: 'og:type', content: 'website' }],
    ['meta', { property: 'og:title', content: '@sylphx/webgpu' }],
    ['meta', { property: 'og:description', content: 'WebGPU for Node.js & Bun via wgpu-rs' }],
  ],

  themeConfig: {
    logo: '/logo.svg',

    nav: [
      { text: 'Guide', link: '/guide/' },
      { text: 'API', link: '/api/' },
      { text: 'Examples', link: '/examples/' },
      { text: 'GitHub', link: 'https://github.com/SylphxAI/webgpu' },
    ],

    sidebar: {
      '/guide/': [
        {
          text: 'Introduction',
          items: [
            { text: 'What is WebGPU?', link: '/guide/' },
            { text: 'Why This Library?', link: '/guide/why' },
            { text: 'Getting Started', link: '/guide/getting-started' },
            { text: 'First Steps', link: '/guide/first-steps' },
          ],
        },
        {
          text: 'Core Concepts',
          items: [
            { text: 'GPU & Adapters', link: '/guide/gpu-adapters' },
            { text: 'Buffers', link: '/guide/buffers' },
            { text: 'Textures', link: '/guide/textures' },
            { text: 'Shaders', link: '/guide/shaders' },
            { text: 'Pipelines', link: '/guide/pipelines' },
          ],
        },
        {
          text: 'Advanced',
          items: [
            { text: 'Compute Shaders', link: '/guide/compute' },
            { text: 'Rendering', link: '/guide/rendering' },
            { text: 'Performance', link: '/guide/performance' },
            { text: 'Testing', link: '/guide/testing' },
          ],
        },
      ],
      '/api/': [
        {
          text: 'API Reference',
          items: [
            { text: 'Overview', link: '/api/' },
            { text: 'GPU', link: '/api/gpu' },
            { text: 'Adapter', link: '/api/adapter' },
            { text: 'Device', link: '/api/device' },
            { text: 'Buffer', link: '/api/buffer' },
            { text: 'Texture', link: '/api/texture' },
            { text: 'Pipeline', link: '/api/pipeline' },
            { text: 'Command Encoder', link: '/api/command-encoder' },
          ],
        },
      ],
      '/examples/': [
        {
          text: 'Examples',
          items: [
            { text: 'Overview', link: '/examples/' },
            { text: 'Compute', link: '/examples/compute' },
            { text: 'Triangle', link: '/examples/triangle' },
            { text: 'Texture Upload', link: '/examples/texture-upload' },
            { text: 'Render Bundle', link: '/examples/render-bundle' },
          ],
        },
      ],
    },

    socialLinks: [
      { icon: 'github', link: 'https://github.com/SylphxAI/webgpu' },
    ],

    footer: {
      message: 'Released under the MIT License.',
      copyright: 'Copyright © 2024 SylphX AI',
    },

    search: {
      provider: 'local',
    },

    editLink: {
      pattern: 'https://github.com/SylphxAI/webgpu/edit/main/docs/:path',
      text: 'Edit this page on GitHub',
    },
  },

  markdown: {
    lineNumbers: true,
  },
})

import { defineConfig } from 'astro/config';
import node from '@astrojs/node';


// https://astro.build/config
export default defineConfig({
  output: "server",
  vite: {
    server: {
      hmr: {
        host: "localhost",
        port: 8900
      }
    },
  },
  adapter: node({
    mode: "standalone"
  }),
});
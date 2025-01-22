import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import { fileURLToPath, URL } from "node:url";


// https://vitejs.dev/config/
export default defineConfig({
  base: "./",
  plugins: [vue()],
  resolve: {
    alias: {
      "@": fileURLToPath(new URL("./src", import.meta.url))
    }
  },
  server: {
    port: 10000,
    proxy: {
      "/api": {
        target: "http://127.0.0.1:18080/",	//target: " http://localhost:8081",
        changeOrigin: true
      },
    }
  }
});

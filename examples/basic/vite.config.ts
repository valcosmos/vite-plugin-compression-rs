import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import {
  Algorithm,
  CompressionType,
  vitePluginCompression,
} from 'vite-plugin-compression-rs'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    // gizp
    vitePluginCompression({
      algorithm: Algorithm.Gzip,
      ext: '.gz',
      compressionOptions: {
        compressionType: CompressionType.Best,
      },
    }),
    // br
    vitePluginCompression({
      algorithm: Algorithm.BrotliCompress,
      ext: '.br',
      compressionOptions: {
        level: 11,
      },
    }),
  ],
})

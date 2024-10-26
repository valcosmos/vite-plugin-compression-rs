import vue from '@vitejs/plugin-vue'
import jsx from '@vitejs/plugin-vue-jsx'
import { vitePluginCompression, Algorithm } from 'vite-plugin-compression-rs'

export default () => {
  return {
    build: {
      assetsInlineLimit: 0,
    },
    plugins: [
      vue(),
      jsx(),
      // gizp
      vitePluginCompression(),
      // br
      vitePluginCompression({
        algorithm: Algorithm.BrotliCompress,
      }),
    ],
  }
}

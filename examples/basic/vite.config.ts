import vue from '@vitejs/plugin-vue'
import jsx from '@vitejs/plugin-vue-jsx'
import {
  vitePluginCompression,
  Algorithm,
  CompressionType,
} from 'vite-plugin-compression-rs'

export default () => {
  return {
    build: {
      assetsInlineLimit: 0,
    },
    plugins: [
      vue(),
      jsx(),
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
  }
}

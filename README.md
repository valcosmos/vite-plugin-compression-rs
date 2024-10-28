# vite-plugin-compression-rs

**English** | [中文](./README.zh_CN.md)

Use `gzip` or `brotli` to compress resources.

This project is based on [vite-plugin-compression](https://github.com/vbenjs/vite-plugin-compression) and incorporates Rust support. The compression-related logic has been rewritten to enhance performance. By leveraging Rust's efficiency, this plugin aims to provide faster and more optimized compression for your Vite projects.

## Install (yarn or npm)

**node version:** >=12.0.0

**vite version:** >=2.0.0

```
yarn add vite-plugin-compression-rs -D
```

or

```
npm i vite-plugin-compression-rs -D
```

## Usage

- Configuration plugin in vite.config.ts

```ts
import {
  vitePluginCompression,
  Algorithm,
  CompressionType,
} from 'vite-plugin-compression-rs'

export default defineConfig({
  plugins: [
    // ...other plugins
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
```

### Options

| params             | type                                  | default          | default                                                                           |
| ------------------ | ------------------------------------- | ---------------- | --------------------------------------------------------------------------------- |
| verbose            | `boolean`                             | `true`           | Whether to output the compressed result in the console                            |
| filter             | `RegExp or (file: string) => boolean` | `DefaultFilter`  | Specify which resources are not compressed                                        |
| disable            | `boolean`                             | `false`          | Whether to disable                                                                |
| threshold          | `number`                              | `1025`           | It will be compressed if the volume is larger than threshold, the unit is b       |
| algorithm          | `Algorithm`                           | `Algorithm.Gzip` | Compression algorithm, optional ['gzip','brotliCompress' ,'deflate','deflateRaw'] |
| ext                | `string`                              | `.gz`            | Suffix of the generated compressed package                                        |
| compressionOptions | `object`                              | -                | The parameters of the corresponding compression algorithm                         |
| deleteOriginFile   | `boolean`                             | -                | Whether to delete source files after compression                                  |

**DefaultFilter**

`/\.(js|mjs|json|css|html)$/i`

## Example

**Run Example**

```bash

pnpm install

cd ./examples/basic

pnpm test:gzip

pnpm test:br

```

## License

MIT

## Inspiration

[vite-plugin-compression](https://github.com/vbenjs/vite-plugin-compression)

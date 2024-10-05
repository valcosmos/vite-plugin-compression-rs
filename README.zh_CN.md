# vite-plugin-compression-rs

**中文** | [English](./README.md)

使用 `gzip` 或者 `brotli` 来压缩资源.

本项目基于 [vite-plugin-compression](https://github.com/vbenjs/vite-plugin-compression)，并增加了 Rust 支持。压缩相关的逻辑已经被重写以提高性能。通过利用 Rust 的高效性，本插件旨在为您的 Vite 项目提供更快速、更优化的压缩。

## 安装 (yarn or npm)

**node version:** >=12.0.0

**vite version:** >=2.0.0

```
yarn add vite-plugin-compression-rs -D
```

or

```
npm i vite-plugin-compression-rs -D
```

## 使用

- vite.config.ts 中的配置插件

```ts
import viteCompression from 'vite-plugin-compression-rs';

export default () => {
  return {
    plugins: [viteCompression()],
  };
};
```

### 配置说明

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| verbose | `boolean` | `true` | 是否在控制台输出压缩结果 |
| filter | `RegExp or (file: string) => boolean` | `DefaultFilter` | 指定哪些资源不压缩 |
| disable | `boolean` | `false` | 是否禁用 |
| threshold | `number` | - | 体积大于 threshold 才会被压缩,单位 b |
| algorithm | `string` | `gzip` | 压缩算法,可选 [ 'gzip' , 'brotliCompress' ,'deflate' , 'deflateRaw'] |
| ext | `string` | `.gz` | 生成的压缩包后缀 |
| compressionOptions | `object` | - | 对应的压缩算法的参数 |
| deleteOriginFile | `boolean` | - | 压缩后是否删除源文件 |

**DefaultFilter**

`/\.(js|mjs|json|css|html)$/i`

## 示例

**运行示例**

```bash

pnpm install

cd ./examples/basic

pnpm test:gzip

pnpm test:br

```

## License

MIT

## 灵感

[vite-plugin-compression](https://github.com/vbenjs/vite-plugin-compression)
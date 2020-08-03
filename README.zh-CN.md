[![license](https://img.shields.io/github/license/zhangzhilinx/imghasher)](https://choosealicense.com/licenses/mit/)
[![release](https://github.com/zhangzhilinx/imghasher/workflows/release/badge.svg)](https://github.com/zhangzhilinx/imghasher/releases)
[![version](https://img.shields.io/github/v/release/zhangzhilinx/imghasher?color=orange&label=version)](https://github.com/zhangzhilinx/imghasher/releases)
[![open issues](https://img.shields.io/github/issues-raw/zhangzhilinx/imghasher.svg)](https://github.com/zhangzhilinx/imghasher/issues)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-peru.svg)](https://github.com/zhangzhilinx/imghasher/pulls)

# imghash: 图像哈希生成器

[English](README.md)
[简体中文](README.zh-CN.md)

一个使用Rust开发的图像哈希生成小工具，支持简单重命名模式

输出格式为hex字符串（默认小写）或base64字符串

<br>

## 安装

* 从 [Github Release](https://github.com/zhangzhilinx/imghasher/releases) 页面下载预编译二进制程序

* 下载源码并编译
```bash
git clone https://github.com/zhangzhilinx/imghasher.git
cd imghasher
cargo install --path .
imghasher [ARGS]...
```

* 即将支持 `cargo install imghasher` 方式安装

<br>

## 用法

```out
imghasher 0.1.0
ZhangZhilin <corex_public@outlook.com>
A tool developed in Rust to process image hash

USAGE:
    imghasher.exe [FLAGS] [OPTIONS] <FILE>...

FLAGS:
    -b, --base64       Output in base64
    -h, --help         Prints help information
    -q, --quiet        No output, suitable for rename mode
    -R, --recursive    Process directories recursively
        --rename       Rename the image file name to the corresponding hash
    -U, --upper        Output in uppercase, ignored in base64 mode
    -V, --version      Prints version information

OPTIONS:
        --algo <algorithm>    Choose a hash algorithm [default: dhash]  [values: ahash, dct_ahash, dhash, dct_dhash]

ARGS:
    <FILE>...    Sets the input files or directories to use

```

支持的图像哈希算法有:
* `ahash`: 平均哈希 (Average Hash)
* `dct_ahash`: DCT处理后的aHash，也就是所谓的 *pHash*
* `dhash`: 差异哈希 / 梯度哈希 (Difference Hash / Gradient Hash)
* `dct_dhash`: DCT处理后的dHash

> 然而dct_ahash算法的实现输出的结果有些怪异，
> 建议使用dct_dhash代替

<br>

## 贡献

1. Fork
2. 创建你的feature分支: `git checkout -b my-new-feature`
3. 提交你的修改: `git commit -am 'feat: add some feature'`
4. 推送对应分支: `git push origin my-new-feature`
5. 提交一个pull request请求 :D

<br>

## 协议

MIT © zhangzhilinx

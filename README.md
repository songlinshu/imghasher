[![license](https://img.shields.io/github/license/zhangzhilinx/imghasher)](https://choosealicense.com/licenses/mit/)
[![release](https://github.com/zhangzhilinx/imghasher/workflows/release/badge.svg)](https://github.com/zhangzhilinx/imghasher/releases)
[![version](https://img.shields.io/github/v/release/zhangzhilinx/imghasher?color=orange&label=version)](https://github.com/zhangzhilinx/imghasher/releases)
[![open issues](https://img.shields.io/github/issues-raw/zhangzhilinx/imghasher.svg)](https://github.com/zhangzhilinx/imghasher/issues)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-peru.svg)](https://github.com/zhangzhilinx/imghasher/pulls)

# imghasher: Image Hash Generator

[English](README.md)
[简体中文](README.zh-CN.md)

A small tool for image hash generation developed using Rust, supports simple renaming mode

The output format is hex string (default lowercase) or base64 string

<br>

## Installation

* Download pre-compiled binaries from the [Github Release](https://github.com/zhangzhilinx/imghasher/releases) page

* Download source code and compile it
```bash
git clone https://github.com/zhangzhilinx/imghasher.git
cd imghasher
cargo install --path .
imghasher [ARGS]...
```

* Soon `cargo install imghasher` will be supported!

<br>

## Usage

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

The supported hash algorithms are:
* `ahash`: Average Hash
* `dct_ahash`: DCT-processed aHash, or *pHash*
* `dhash`: Difference Hash / Gradient Hash
* `dct_dhash`: DCT-processed dHash

> However, the output of the dct_ahash algorithm implementation is a bit weird, 
> so we suggest using dct_dhash instead.

<br>

## Contributing

1. Fork it!
2. Create your feature branch: `git checkout -b my-new-feature`
3. Commit your changes: `git commit -am 'feat: add some feature'`
4. Push to the branch: `git push origin my-new-feature`
5. Submit a pull request :D

<br>

## License

MIT © zhangzhilinx

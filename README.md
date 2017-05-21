# libdrm-sys

[![CircleCI](https://circleci.com/gh/devurandom/libdrm-sys-rs.svg?style=shield)](https://circleci.com/gh/devurandom/libdrm-sys-rs)
[![crates.io](https://img.shields.io/crates/v/libdrm-sys.svg)](https://crates.io/crates/libdrm-sys)

Native bindings to the libdrm library.

This package contains only the raw C bindings. If you want something more convenient, please have a look at [libdrm-sweet](https://crates.io/crates/libdrm-sweet).

## Requirements

* libdrm >= 2.4.58
  - You can select higher minimum versions by setting e.g. `--feature=version_2_4_67`
  - `libdrm-dev` on Debian/Ubuntu
* C headers
  - `build-essential` on Debian/Ubuntu
* pkg-config

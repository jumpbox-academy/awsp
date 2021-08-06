# AWSP - CLI To Manage your AWS Profiles!
AWSP provide a interactive terminal to interact with your AWS Profile. The aim of this project is to make it easier to navigate, observe and manage your AWS Profile in the wild. 

---

[![release](https://img.shields.io/github/v/release/kubeopsskills/awsp?logo=awsp)](https://github.com/kubeopsskills/awsp/releases)
[![License Apache](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://github.com/kubeopsskills/awsp/blob/beta/LICENSE-APACHE)
[![License MIT](https://img.shields.io/badge/license-MIT-green?label=License)](https://github.com/kubeopsskills/awsp/blob/beta/LICENSE-MIT)
[![Download](https://img.shields.io/github/downloads/kubeopsskills/awsp/total)](https://github.com/kubeopsskills/awsp/releases)

---


## Demo
![screenshot1](./assets/images/select-profile.png)
![screenshot2](./assets/images/select-region.png)

## Prerequisite
- rust: [rust](https://www.rust-lang.org/tools/install)
- upx: [upx](https://upx.github.io/)

## Getting Started
- Clone the repository to your local machine.
- `cd` to the root of project folder.
### Build
```bash
make all
```

### Link binary to $Path
```bash
ln -s /target/release/awsp ~/usr/local/bin/awsp
```


## Usage
```bash
USAGE:
    awsp [OPTIONS]

FLAGS:
    -h, --help    Prints help information

OPTIONS:
    -c, --config <config>      Override a aws configuration file (default = ~/.aws/config)
    -r, --region <region>      Region Selector
    -v, --version <version>    Print version info and exit
```

## Contributing

We'd love your help! Please see [CONTRIBUTING][contrib] to learn about the
kinds of contributions we're looking for.


## Todo
- We use [GitHub Issues][github-issue] to track our to do items.
- Please check the [following link][follow] if you would like to contribute to the project.

## CHANGELOG
See [CHANGELOG][changelog]


[contrib]: https://github.com/kubeopsskills/awsp/blob/beta/CONTRIBUTING.md
[follow]: https://github.com/kubeopsskills/awsp/blob/beta/CONTRIBUTING.md
[changelog]: https://github.com/kubeopsskills/awsp/blob/beta/CHANGELOG.md
[github-issue]: https://github.com/kubeopsskills/awsp/issues/new





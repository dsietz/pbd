[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Coverage Status](https://coveralls.io/repos/github/dsietz/pbd/badge.svg?branch=master)](https://coveralls.io/github/dsietz/pbd?branch=master)
[![Docs.rs](https://docs.rs/pbd/badge.svg)](https://docs.rs/pbd)

Linux: [![Build Status](https://travis-ci.org/dsietz/pbd.svg?branch=master)](https://travis-ci.org/dsietz/pbd)
Windows: [![Build status](https://ci.appveyor.com/api/projects/status/o3w8863fpji4pfoq?svg=true)](https://ci.appveyor.com/project/dsietz/pbd/branch/master)


# Privacy by Design (PbD) SDK

For software development teams who implement Privacy by Design practices, this PbD SDK provides enablers to help you easily and transparently applying best practices. Unlike other solutions, this SDK maps directly to the Data Privacy strategies to provide a complete tool kit and saves developers time from having to search, derive, or piece together disparate solutions.

---

**Table of Contents**
- [Privacy by Design (PbD) SDK](#privacy-by-design-pbd-sdk)
  - [What's New](#whats-new)
  - [Features](#features)
  - [About](#about)
  - [How to Contribute](#how-to-contribute)
  - [License](#license)

## What's New

Here's whats new in 0.2.0:

We've upgraded to more current versions of our crate's dependencies
- actix-rt to 1.1
- actix-web to 2.0
- actix-service to 1.0
- base64 to 0.12
- futures to 0.3
- json to 0.12
- rayon to 1.3

## Features

- Data Usage Agreements (dua)
- Data Tracker Chain (dtc)
- Data Security Guard (dsg)

## About

The intent of the `pbd` development kit is to enable the implementation of [privacy design strategies and tactics](./docs/DESIGN-STRATEGIES.md) by providing the functionality and components for developers to implement best practices in their own software soltuions. 

## How to Contribute

Details on how to contribute can be found in the [CONTRIBUTING](./CONTRIBUTING.md) file.

## License

`pbd` is primarily distributed under the terms of the Apache License (Version 2.0).

See [LICENSE-APACHE "Apache License](./LICENSE-APACHE) for details.
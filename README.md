[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Coverage Status](https://coveralls.io/repos/github/dsietz/pbd/badge.svg?branch=master)](https://coveralls.io/github/dsietz/pbd?branch=master)
[![Discussions](https://img.shields.io/github/discussions/dsietz/pbd)](https://github.com/dsietz/pbd/discussions)
[![Docs.rs](https://docs.rs/pbd/badge.svg)](https://docs.rs/pbd)

Linux: [![Build Status](https://github.com/dsietz/pbd/workflows/Master/badge.svg)](https://github.com/dsietz/pbd/actions?query=workflow%3AMaster)
Windows: [![Build status](https://ci.appveyor.com/api/projects/status/o3w8863fpji4pfoq?svg=true)](https://ci.appveyor.com/project/dsietz/pbd/branch/master)


# Privacy by Design (PbD) SDK

For software development teams who implement Privacy by Design practices, this PbD SDK provides enablers to help you easily and transparently applying best practices. Unlike other solutions, this SDK maps directly to the Data Privacy strategies to provide a complete tool kit and saves developers time from having to search, derive, or piece together disparate solutions.

---

**Table of Contents**
- [Privacy by Design (PbD) SDK](#privacy-by-design-pbd-sdk)
  - [What's New](#whats-new)
  - [Features](#features)
  - [Examples](#examples)
      - [Data Privacy Inspector](#data-privacy-inspector)
      - [Data Security Guard](#data-security-guard)
      - [Data Tracker Chain](#data-tracker-chain)
      - [Data Usage Agreement](#data-usage-agreement)
  - [About](#about)
  - [How to Contribute](#how-to-contribute)
  - [License](#license)

## What's New

Here's whats new in 0.5.0:

We've made breaking changes in this newest version 0.5.0!
1. Due to the following reasons, we have dropped the `extractor` and `middleware` features for the Data Tracker Chain and Data Usage Agreement features. (Resolves Isses [#45](https://github.com/dsietz/pbd/issues/45), [#46](https://github.com/dsietz/pbd/issues/46), and [#49](https://github.com/dsietz/pbd/issues/49))
   - focus on remaining a light-weight and flexible SDK
   - incompatibility issues with `actix-web version ~4` 
   - improved Mean Time To Resolve (MTTR) issues
   > NOTE: The examples will still provide demonstration of how to implement these features using `actix-web` without axtractors or middleware. 
2.  Updated `regex` version to fix security vulnerability

## Features

- Data Usage Agreements (dua)
- Data Tracker Chain (dtc)
- Data Privacy Inspector (dpi)
- Data Security Guard (dsg)

## Examples

This SDK comes with the executable examples for each of the features. The code for these examples can be found [here](https://github.com/dsietz/pbd/tree/master/examples).

#### Data Privacy Inspector
1. From the command line terminal, start the service using: `cargo run --example data-privacy-inspector`
2. Then make the following http request
```
POST / HTTP/1.1
Host: localhost:8088
Content-Type: plain/text
Content-Length: 610

Dear Aunt Bertha,

I can't believe it has already been 10 years since we moved to back to the Colorado. 
I love Boulder and haven't thought of leaving since. So please don't worry when I tell you that we are moving in less than a week.
We will be upgrading to a larger home on the other side of the city on Peak Crest Lane. 
It have a great view of the mountains and we will have a two car garage.

We will have the same phone number, so you can still reach us. But our new address with be 1345 Peak Crest Lane Boulder, Colorado 125468.

Let us know if you ever want to vist us. 

Sincerely,
Robert
```

#### Data Security Guard
1. From the command line terminal, start the service using: `cargo run --example data-security-guard`
2. Then make the following http request
```
GET / HTTP/1.1
Host: localhost:8088
Content-Type: application/json
Content-Length: 1097

{"encrypted_data":[130,37,248,85,153,227,79,249,207,97,173,90,24,95,190,46],"encrypted_symmetric_key":[50,133,49,31,191,107,92,185,73,215,226,59,30,241,210,149,177,158,166,200,98,86,22,245,251,224,49,239,177,245,236,43,255,190,251,162,47,218,206,2,72,253,181,24,143,32,41,233,13,35,195,225,155,110,95,59,223,209,126,134,218,58,45,97,40,184,148,184,188,141,143,235,131,154,76,1,246,8,19,107,226,71,148,231,196,209,197,85,151,36,203,107,125,168,145,93,57,217,188,71,211,239,3,25,230,27,165,65,191,250,178,21,248,49,70,199,34,91,62,22,5,50,50,180,134,31,137,30,155,215,253,109,46,220,209,218,50,98,194,151,63,8,4,164,100,225,94,122,81,93,130,170,255,168,186,76,251,163,179,250,169,167,52,158,223,187,170,101,66,108,22,153,195,140,203,149,243,129,137,161,246,115,156,87,140,96,163,209,169,244,175,34,150,216,43,234,24,7,220,197,87,65,196,43,230,223,61,7,47,171,193,239,121,46,208,245,161,188,113,49,216,205,147,122,233,136,24,58,157,99,54,188,100,14,19,55,11,218,199,148,3,2,74,148,5,174,155,118,136,64,210,182,101,50,168,74],"nonce":[100,109,70,86,87,48,111,104,67,71,78,54,66,74,114,48],"padding":1}
```

#### Data Tracker Chain
1. From the command line terminal, start the service using: `cargo run --example data-tracker-chain`
2. Then make the following http request
```
GET / HTTP/1.1
Host: localhost:8088
Content-Type: application/json
Data-Tracker-Chain: W3siaWRlbnRpZmllciI6eyJkYXRhX2lkIjoib3JkZXJ+Y2xvdGhpbmd+aVN0b3JlfjE1MTUwIiwiaW5kZXgiOjAsInRpbWVzdGFtcCI6MCwiYWN0b3JfaWQiOiIiLCJwcmV2aW91c19oYXNoIjoiMCJ9LCJoYXNoIjoiMjcyMDgxNjk2NjExNDY0NzczNzI4MDI0OTI2NzkzNzAzMTY3NzgyIiwibm9uY2UiOjV9LHsiaWRlbnRpZmllciI6eyJkYXRhX2lkIjoib3JkZXJ+Y2xvdGhpbmd+aVN0b3JlfjE1MTUwIiwiaW5kZXgiOjEsInRpbWVzdGFtcCI6MTU3ODA3MTIzOSwiYWN0b3JfaWQiOiJub3RpZmllcn5iaWxsaW5nfnJlY2VpcHR+ZW1haWwiLCJwcmV2aW91c19oYXNoIjoiMjcyMDgxNjk2NjExNDY0NzczNzI4MDI0OTI2NzkzNzAzMTY3NzgyIn0sImhhc2giOiI1MDEwNDE0OTcwMTA5ODcwMDYzMjUxMTE0NDEyNTg2NzczNjE5MyIsIm5vbmNlIjo1fV0=
```

#### Data Usage Agreement
1. From the command line terminal, start the service using: `cargo run --example data-usage-agreement`
2. Then make the following http request
```
GET / HTTP/1.1
Host: localhost:8088
Content-Type: application/json
Data-Usage-Agreement: [{"agreement_name":"billing","location":"https://github.com/dsietz/pbd/blob/master/tests/duas/Patient%20Data%20Use%20Agreement.pdf","agreed_dtm": 1553988607}]
```

## About

The intent of the `pbd` development kit is to enable the implementation of [privacy design strategies and tactics](./docs/DESIGN-STRATEGIES.md) by providing the functionality and components for developers to implement best practices in their own software soltuions. 

## How to Contribute

Details on how to contribute can be found in the [CONTRIBUTING](./CONTRIBUTING.md) file.

## License

`pbd` is primarily distributed under the terms of the Apache License (Version 2.0).

See [LICENSE-APACHE "Apache License](./LICENSE-APACHE) for details.
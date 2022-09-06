# users-rs - Monatize Microservice Architecture

# Table of Contents
- [Introduction](#introduction)
- [Tech Stack](#tech-stack)
- [Contributing](#contributing)
- [License](#license)

--- 

# Introduction
users-rs is a part of the Monatize Microservice Architecture.

This microservice was built to handle user signup, login, and authentication.

# Tech Stack
**Language** - [Rust](https://www.rust-lang.org/)
**Web Server Library** - [Axum](https://github.com/tokio-rs/axum)
**Important Deps**
- [prisma-client-rust](https://github.com/Brendonovich/prisma-client-rust) by [Brendonovich](https://github.com/Brendonovich)
    - Make transactions & queries to the PostgresQL DB's
- [rust-web3](https://github.com/tomusdrw/rust-web3) by [tomusdrw](https://github.com/tomusdrw)
    - Used to verify signatures w/ Ethereum

# Contributing
If you'd like to propose a change to this repo, first open up an [Issue](https://github.com/Monatize/users-rs/issues) to make sure you're not wasting your time

Then, fork this repo into your own GitHub account

After you fork and clone the repo to your local machine, please create a new branch detailing your contribution, ex. ``my_new_feature``

When committing changes, please use the [Convential Commits](https://www.conventionalcommits.org/en/v1.0.0/) standard

After you've committed all changes, please open up a PR and detail what changes the PR makes, and what issue it's solving.

# License
Copyright 2022 Monatize Labs LLC

The Monatize users-rs code is licensed under the AGPLv3: https://opensource.org/licenses/agpl-3.0 This license file can also be found in [LICENSE](./LICENSE) in the root of the source tree.
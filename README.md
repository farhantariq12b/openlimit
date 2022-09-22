<p align="center">
  <img width="400px" src="logo.svg" />
</p>

![build](https://github.com/nash-io/openlimits/workflows/Rust/badge.svg) ![security](https://github.com/nash-io/openlimits/workflows/Security%20audit/badge.svg)

A open source Rust high performance cryptocurrency trading API with support for multiple exchanges and language wrappers. Focused in safety, correctness and speed.

## Project goals

This is an ambitious project that seeks to create a new industry standard API implementation for secure, correct and high performance cryptocurrency trading. It is initially focused on spot exchanges as a method to define how initial data structures and project architecture should be to allow zero cost abstractions around the exchanges' peculiarities. So initial project goals are:

* Based on Rust, memory safe by default.
* Support for websockets and user defined networking.
* Thin layer wrappers for Java, C#, Python and Node.js
* Easy to add support for additional exchanges.
* Open-source only, now and forever.

Future goals are:

* Support for futures trading
* Support for options trading
* WASM compilation allowing it to be embedded in static web pages

**Warning**: the project is still in development and a lot of breaking changes are being made.


### Community

[Click here](https://discord.gg/rSTDX5fuNF) to access our Discord Community.

### Testing

In order to run the tests you will have to provide environment variables for the sandbox API of the exchanges, you can use environment variables or use a `.env` file.

### Sponsorship

We invite industry participants to join us in sponsoring a new high quality open source standard for crypto trading APIs. [Nash](https://nash.io) is dedicating a maintainer and initial rewards for external contributors that close issues. Look for the wiki [rewards table](https://github.com/nash-io/openlimits/wiki/Rewards-sizes) and for reward size labels on open issues.

# Ferinth

[![Made with Rust](https://img.shields.io/badge/Made_with-Rust-b11522?labelColor=e82833&logo=rust)](https://www.rust-lang.org)
[![license - MIT](https://img.shields.io/github/license/gorilla-devs/ferinth)](https://github.com/gorilla-devs/ferinth/blob/master/LICENSE.txt)

Ferinth is a simple library for using the [Modrinth REST API](https://docs.modrinth.com/api) in Rust.
It uses [reqwest](https://docs.rs/reqwest) for sending requests and deserialising responses to strongly typed structs using [Serde](https://serde.rs).

## About this fork

This fork includes extra bug fixes, updated dependencies, and additional patch improvements over the original crate.

## Features

- Strongly typed structures for API responses
- Useful examples in the method documentations
- Implementations for the following API calls
  - TODO

## Missing Features

- Requests that require large body data
- Better organisation of API calls

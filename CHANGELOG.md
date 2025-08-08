# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0] - 2024-08-27

### Added

- Docs: Added initial documentation drop
- Feature: A script (`run-examples.sh`) to run and display the status (pass/fail) of each example
- Chore: Added `openssl` and `rustls` feature flags to toggle TLS implementation, with `openssl` enabled by default

### Changed

- Update Rusty Falcon to use the latest OpenAPI spec (2025-07-08T00:00:04Z)
- Update example code to follow the latest OpenAPI spec changes
- Fix links in the README
- Fix spelling for docs and api files
- Error hierarchy revamp to structured errors based on `thiserror` crate
- Handle `intel_indicators` example case in `run-examples.sh` script
- Update `rust-version` to `1.82.0`
- Display number of passed examples/tests in `run-examples.sh` script
- Enable `run-example.sh` script to run a single example
- Set certain `DetectsPeriodExternalAlert` model fields to optional

- Upgrade dependencies:
  - `clap` to `4.5.18`
  - `openssl` to `0.10.73`
  - `openssl-sys` to `0.9.109`
  - `reqwest` to `0.12.20`
  - `serde` to `1.0.210`
  - `serde_derive` to `1.0.210`
  - `serde_json` to `1.0.128`
  - `tokio` to `1.40.0`
  - `url` to `2.5.2`

## [0.3.3] - 2023-09-04

### Added

- Example: Falcon Custom IOAs
- Example: Discover API Host Details
- Example: Falcon ZTA example
- Change log using Keep a Changelog format

### Fixed

- Use `MsaspecPeriodMetaInfo` in `DomainPeriodAssessmentsResponse`:`Meta` field

## [<= 0.3.2] - Historical

- Support for Falcon API

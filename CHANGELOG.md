# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-01-01

### Added

- `to_nix` — serialize any `Serialize` value to a Nix expression string
- `to_nix_module` — wrap serialized output in a NixOS module
- `ron_to_nix` / `ron_to_nix_module` — parse RON and convert to Nix
- `escape_nix_string` — escape string literals for Nix
- Optional `cli` feature with reusable clap arguments (`RonixArgs`)
- Nix library: `toRON`, `fromRON`, `importRON`, `mkRON`, `isRONType`
- NixOS helper: `mkRonService` for systemd service generation
- Nix flake with build, checks, and dev shell

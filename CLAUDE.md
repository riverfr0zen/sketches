# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

Documentation has been split into focused files for better context management. See the `claude_docs/` directory for detailed information:

## General Information
- **[overview.md](claude_docs/overview.md)** - Repository overview and package structure
- **[build_commands.md](claude_docs/build_commands.md)** - Build instructions for native and WASM targets
- **[important_notes.md](claude_docs/important_notes.md)** - Critical information, known issues, and workarounds

## Architecture & Systems
- **[package_structure.md](claude_docs/package_structure.md)** - Package organization and module layout
- **[grid_utilities.md](claude_docs/grid_utilities.md)** - Grid utilities for grid-based generative art sketches. When working with code that uses `notan_sketches/src/gridutils.rs`, go through these docs.
- **[emotion_system.md](claude_docs/emotion_system.md)** - Text-to-emotion analysis and color mapping system
- **[shader_system.md](claude_docs/shader_system.md)** - Custom shader utilities and hot reloading
- **[dependencies.md](claude_docs/dependencies.md)** - Target-specific dependency configuration

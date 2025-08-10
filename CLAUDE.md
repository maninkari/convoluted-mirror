# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Convoluted Mirror is a WebAssembly-based experimental mirror application that uses Rust for image processing and JavaScript for the web interface. The project applies convolution kernels to webcam video feeds to create artistic mirror effects.

## Development Commands

**Prerequisites:**
- Install Rust: https://www.rust-lang.org/tools/install
- Install wasm-pack: `cargo install wasm-pack`
- Install yarn: for JavaScript dependencies

**Essential Commands:**
- `yarn` - Install JavaScript dependencies
- `yarn wasm` - Build the WebAssembly module (required after Rust changes)
- `yarn build` - Build the complete application for production
- `yarn start` - Start development server with hot reload
- `yarn format` - Format Rust code (requires `cargo fmt` and `cargo clippy`)

**Development Workflow:**
1. After modifying Rust code in `src/`, run `yarn wasm` to rebuild WASM
2. Run `yarn start` to serve the application locally
3. For production builds, use `yarn build` which outputs to `dist/`

## Architecture

**Hybrid Language Architecture:**
- **Rust/WASM** (`src/`): High-performance image processing and convolution operations
- **JavaScript** (`index.js`): DOM manipulation, WebRTC camera access, and application lifecycle
- **Webpack**: Bundles JavaScript and handles WASM loading with experimental asyncWebAssembly support

**Key Components:**

**Rust Side:**
- `lib.rs`: Main Mirror struct with multiple convolution methods exposed to JavaScript
- `frame.rs`: Frame processing utilities including delta frame computation and pixel manipulation
- WASM bindings via `wasm-bindgen` for JavaScript interoperability

**JavaScript Side:**
- `index.js`: Main application entry point handling camera setup, canvas management, and WASM integration
- Two canvas elements: `mirrorCanvas` (direct video feed) and `mirrorConvolute` (processed output)
- Real-time processing loop using `requestAnimationFrame`

**Video Processing Pipeline:**
1. WebRTC captures user camera feed
2. JavaScript draws video frames to `mirrorCanvas`
3. Rust processes frames through convolution kernels
4. Processed frames render to `mirrorConvolute` canvas

**Convolution System:**
- Multiple kernel configurations available (see commented examples in `lib.rs`)
- Dynamic kernel passing from JavaScript to Rust via `convolute_kernel()` method
- Delta frame processing compares consecutive frames for motion detection
- Configurable color channels via bit flags in convolution methods

## File Structure

- `src/` - Rust source code for WASM module
- `app/` - HTML templates and CSS
- `pkg/` - Generated WASM artifacts (auto-generated, don't edit)
- `index.js` - JavaScript application entry point
- `webpack.config.js` - Build configuration with WASM support

## Development Notes

- The `pkg/` directory contains auto-generated WASM bindings - never edit these files directly
- After Rust changes, always run `yarn wasm` before testing
- Canvas dimensions are hardcoded to 1100x600 in `index.js`
- The application requires camera permissions to function
- Fullscreen functionality is implemented with cross-browser compatibility
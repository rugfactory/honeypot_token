#!/bin/bash

# Build the WASM binary with reproducible build settings
cargo near build reproducible-wasm

# Generate the ABI (Application Binary Interface)
# cargo near abi

echo "Build completed successfully!"
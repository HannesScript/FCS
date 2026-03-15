#!/usr/bin/env bash
set -euo pipefail

# Ensure Cargo binaries are on PATH for this script.
export PATH="$HOME/.cargo/bin:$PATH"

if ! command -v cargo >/dev/null 2>&1; then
	echo "cargo is not installed. Install Rust toolchain first and rerun this script."
	exit 1
fi

# Install espup if missing.
if ! command -v espup >/dev/null 2>&1; then
	cargo install espup --locked
fi

ESP_ENV_SCRIPT="$HOME/export-esp.sh"

# Install ESP toolchains if linker is unavailable.
if ! command -v xtensa-esp32s3-elf-gcc >/dev/null 2>&1; then
	espup install
fi

if [[ ! -f "$ESP_ENV_SCRIPT" ]]; then
	echo "Missing ESP environment script: $ESP_ENV_SCRIPT"
	echo "espup install did not produce the expected script."
	exit 1
fi

# Load Xtensa toolchain binaries and env vars provided by espup.
# shellcheck disable=SC1090
source "$ESP_ENV_SCRIPT"

# Persist env script for future shells.
if [[ -f "$HOME/.bashrc" ]] && ! grep -q 'export-esp\.sh' "$HOME/.bashrc"; then
	echo 'source "$HOME/export-esp.sh"' >> "$HOME/.bashrc"
fi

# Optional flasher tool.
if ! command -v espflash >/dev/null 2>&1; then
	cargo install espflash --version 3.3.0
fi

if ! command -v xtensa-esp32s3-elf-gcc >/dev/null 2>&1; then
	echo "xtensa-esp32s3-elf-gcc is still missing after setup."
	exit 1
fi

echo "Linker: $(command -v xtensa-esp32s3-elf-gcc)"

#!/usr/bin/env bash
set -euo pipefail

ESP_ENV_SCRIPT="$HOME/export-esp.sh"

if [[ -f "$ESP_ENV_SCRIPT" ]]; then
	# Load Xtensa toolchain binaries and env vars provided by espup.
	# shellcheck disable=SC1090
	source "$ESP_ENV_SCRIPT"
	export PATH="$HOME/.cargo/bin:$PATH"
else
	echo "Missing ESP environment script: $ESP_ENV_SCRIPT"
	echo "Install and configure espup, then retry."
	exit 1
fi

if command -v espflash >/dev/null 2>&1; then
	cargo run
else
	echo "espflash is not installed or not on PATH."
	echo "Install it with: cargo install espflash --version 3.3.0"
	exit 127
fi

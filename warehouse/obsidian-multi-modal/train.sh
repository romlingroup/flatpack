#!/bin/bash

# Get the directory where the script is located
export SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo -e "🚀 train.sh is running in: $SCRIPT_DIR\n"

# === BEGIN USER CUSTOMIZATION ===
export REPO_NAME=obsidian
export FLATPACK_NAME=obsidian-multi-modal
# === END USER CUSTOMIZATION ===

source "$SCRIPT_DIR/device.sh" || {
  echo "😱 Error: Failed to source device.sh" >&2
  exit 1
}

# Required devices (cpu cuda mps)
REQUIRED_DEVICES="cuda mps"

# Check if DEVICE is among the required devices
if [[ ! " $REQUIRED_DEVICES " =~ " $DEVICE " ]]; then
  echo "😱 Error: This script requires one of the following devices: $REQUIRED_DEVICES." >&2
  exit 1
fi

# === BEGIN USER CUSTOMIZATION ===
# Temporary workaround for macOS
if [ "$OS" = "Darwin" ]; then
  pip install flash-attn --no-build-isolation
  pip install ninja
  pip install -e .
  pip install --upgrade transformers==4.34.0
else
  "${VENV_PYTHON}" pip install flash-attn --no-build-isolation
  "${VENV_PYTHON}" pip install ninja
  "${VENV_PYTHON}" pip install -e .
  "${VENV_PYTHON}" pip install --upgrade transformers==4.34.0
fi

chmod +x scripts/download_mm_projector.sh
bash scripts/download_mm_projector.sh

"${VENV_PYTHON}" -m llava.serve.cli \
    --model-path NousResearch/Obsidian-3B-V0.5 \
    --image-file "https://github.com/romlingroup/flatpack/tree/main/warehouse/obsidian-multi-modal/tiger.png" \
    --load-4bit
# === END USER CUSTOMIZATION ===

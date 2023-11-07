#!/bin/bash

# Get the directory where the script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo -e "🚀 train.sh is running in: $SCRIPT_DIR\n"

# === BEGIN USER CUSTOMIZATION ===
export REPO_NAME=nanoGPT-colab
export FLATPACK_NAME=nanogpt-scratch
# === END USER CUSTOMIZATION ===

source "$SCRIPT_DIR/device.sh" || {
  echo "⚠️ Error: Failed to source device.sh" >&2
  exit 1
}

# === BEGIN USER CUSTOMIZATION ===
cp train.py train.py.backup
sed -i 's/dtype = "bfloat16"/dtype = "float16"/' train.py
sed -i 's/compile = True/compile = False/' train.py
"${VENV_PYTHON}" data/shakespeare_char/prepare.py
"${VENV_PYTHON}" train.py config/train_shakespeare_char.py
# === END USER CUSTOMIZATION ===

#!/bin/bash

# Check if REPO_NAME is set
if [[ -z "${REPO_NAME}" ]]; then
  echo "Error: REPO_NAME is not set. Please set the REPO_NAME environment variable." >&2
  exit 1
fi

# Check if FLATPACK_NAME is set
if [[ -z "${FLATPACK_NAME}" ]]; then
  echo "Error: FLATPACK_NAME is not set. Please set the FLATPACK_NAME environment variable." >&2
  exit 1
fi

# Check if running in Colab environment
if [[ "${COLAB_GPU}" == "1" ]]; then
  echo "Running in Colab environment"
  IS_COLAB=1
else
  echo "Not running in Colab environment"
  IS_COLAB=0
fi

# Determine the working directory and device based on the environment
if [[ $IS_COLAB -eq 0 ]]; then
  OS=$(uname)
  if [ "$OS" = "Darwin" ]; then
    WORK_DIR="$REPO_NAME.c"
    DEVICE="mps"
  else
    WORK_DIR="/home/content/$REPO_NAME.c"
    DEVICE="cpu"
  fi
else
  WORK_DIR="/content/$FLATPACK_NAME/$REPO_NAME.c"
  DEVICE="cuda"
fi

# Change to the working directory
cd "$WORK_DIR" || {
  echo "Error: Failed to change to directory $WORK_DIR" >&2
  exit 1
}
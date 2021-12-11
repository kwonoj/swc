#!/usr/bin/env bash
set -eu

NODE_PLATFORM_NAME=$(node -e "console.log(require('os').platform())")


(cd scripts/npm/core-$NODE_PLATFORM_NAME && yarn link)
yarn link @swc/core-$NODE_PLATFORM_NAME
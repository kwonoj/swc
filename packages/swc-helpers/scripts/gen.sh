#!/bin/bash
set -eu

./scripts/generator.sh > src/index.js
yarn build

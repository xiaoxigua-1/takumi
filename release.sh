#!/bin/bash

cargo publish -p takumi

bun run release --cwd takumi-napi-core
bun run release --cwd takumi-helpers
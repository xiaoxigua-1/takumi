#!/bin/bash

cargo publish -p takumi

bun publish --cwd takumi-napi-core
bun publish --cwd takumi-helpers
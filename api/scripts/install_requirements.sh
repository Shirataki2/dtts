#!/bin/bash
set -euv
apt -y update
apt -y install libssl-dev
rustup component add rustfmt

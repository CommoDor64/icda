#!/bin/bash

# Source the DFX and Cargo environment scripts
source /root/.local/share/dfx/env
source /root/.cargo/env

echo 'Starting dfx in the background...'
dfx start --background --clean

tail -f /dev/null


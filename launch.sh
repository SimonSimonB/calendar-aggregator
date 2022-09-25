#!/usr/bin/env bash
set -e

pushd frontend
npm run build --dev
popd 

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
cd backend
FRONTEND_PATH=$SCRIPT_DIR/frontend/build/ cargo run
#!/usr/bin/env bash

set -e

RUNTIME_FEATURE=$1

if [[ "$RUNTIME_FEATURE" == "with-snowbridge-runtime" ]]
then
    RUNTIME_DIR="runtime/snowbridge"
elif [[ "$RUNTIME_FEATURE" == "with-rococo-runtime" ]]
then
    RUNTIME_DIR="runtime/rococo"
else
    echo "Missing or invalid runtime feature argument. Pass either \"with-snowbridge-runtime\" or \"with-rococo-runtime\"."
    exit 1
fi

echo "Building runtime with features $RUNTIME_FEATURE,runtime-benchmarks"

cargo build --release \
    --no-default-features \
    --features runtime-benchmarks,$RUNTIME_FEATURE

# TODO: add frame_system here once invalid WeightInfo impl is resolved
PALLETS="assets basic_channel::inbound dot_app erc20_app eth_app incentivized_channel::inbound pallet_balances pallet_timestamp verifier_lightclient"

echo "Generating weights module for $RUNTIME_DIR with pallets $PALLETS"

for pallet in $PALLETS
do
    MODULE_NAME="$(tr -s [:] _ <<< $pallet)_weights"
    # TODO: enable options in comments below once
    # all pallets work in wasm
    #    --execution wasm \
    #    --wasm-execution compiled \
    target/release/artemis benchmark \
        --chain spec.json \
        --pallet "${pallet}" \
        --extrinsic "*" \
        --repeat 20 \
        --steps 50 \
        --output $RUNTIME_DIR/src/weights/$MODULE_NAME.rs
    echo "pub mod $MODULE_NAME;" >> $RUNTIME_DIR/src/weights/tmpmod.rs
done

mv $RUNTIME_DIR/src/weights/tmpmod.rs $RUNTIME_DIR/src/weights/mod.rs

echo "Done!"

#!/bin/bash

# Install cargo if it is not found
command -v cargo
if [ $? -ne 0 ]; then
    echo "Installing cargo..."
    curl https://sh.rustup.rs -sSf | sh
    if [ $? -ne 0 ]; then
        echo "Failed to install cargo"
        exit 1
    fi
else
    echo "Cargo already installed"
fi

cargo test

if [ $? -ne 0 ]; then
    echo "One or more tests failed"
    exit 1
fi

echo "All tests passed"
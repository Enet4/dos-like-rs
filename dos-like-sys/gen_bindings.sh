#!/bin/sh
if ! which bindgen > /dev/null; then
    echo "ERROR: `bindgen` not found. Please install using cargo:"
    echo "    cargo install bindgen"
    exit 1
fi

bindgen_opt="--rust-target 1.64"

headers="dos-like/source/dos.h"

# bindgen 0.68.1
cmd="bindgen $bindgen_opt $headers -o src/bindings.rs"
echo "${cmd}"
${cmd}

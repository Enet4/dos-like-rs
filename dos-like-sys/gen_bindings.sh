#!/bin/sh
if ! which bindgen > /dev/null; then
    echo "ERROR: `bindgen` not found. Please install using cargo:"
    echo "    cargo install bindgen"
    exit 1
fi

bindgen_opt="--rust-target 1.47 --size_t-is-usize"

headers="dos-like/source/dos.h"

cmd="bindgen $bindgen_opt $headers -o src/bindings.rs"
echo "${cmd}"
${cmd}

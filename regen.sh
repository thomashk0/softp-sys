#!/bin/bash

set -eu -o pipefail

out="src/bindings.rs"

echo "Generating API bindings -> $out"
bindgen softfp/softfp.h \
    --whitelist-type "sfloat.*" \
    --whitelist-type "RoundingModeEnum"\
    --whitelist-function "cvt_.*" \
    --whitelist-function ".*_sf32" \
    --whitelist-function ".*_sf64" \
    -o ${out}

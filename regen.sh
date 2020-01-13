#!/bin/bash

set -eu -o pipefail

out="src/bindings.rs"

echo "Generating fst API bindings -> $out"
bindgen softfp/softfp.h -o ${out}

# bindgen fstapi/fstapi.h \
#   --whitelist-function "fstWriter.*" \
#   --whitelist-function "fstReader.*" \
#   --whitelist-type "fst.*" \
#   --opaque-type "FILE" \
#   -o ${out}

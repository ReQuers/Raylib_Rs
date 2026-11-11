#!/usr/bin/env bash
set -euo pipefail

command="${1:-build}"
if (($# > 0)); then
    shift
fi

case "$command" in
    build|run|test|check) ;;
    *)
        echo "Usage: $0 [build|run|test|check] [cargo arguments...]" >&2
        exit 2
        ;;
esac

for tool in cargo cmake gcc g++ make; do
    if ! command -v "$tool" >/dev/null 2>&1; then
        echo "Required tool '$tool' was not found. Install Rust, CMake, and the Linux build dependencies from README.md." >&2
        exit 1
    fi
done

export CMAKE_GENERATOR="${CMAKE_GENERATOR:-Unix Makefiles}"
exec cargo "$command" "$@"

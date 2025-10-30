#!/bin/bash
set -e

cd "$(dirname $0)"

echo "========= cargo check cmemu-lib no-features"
cargo +stable check -p cmemu-lib --no-default-features --all-targets

# Naaah, rechecking cc2650-constants all the time takes ages
#if which jq; then
#  for pkg in $(cargo metadata --no-deps --format-version 1 --no-default-features | jq -r '.packages.[].name'); do
#    echo "============ cargo check ${pkg} no-features"
#    # Check without warnings of things becoming unused for now
#    RUSTFLAGS="-A warnings" cargo +stable check -p $pkg --no-default-features
#  done
#fi

echo "========= rustfmt check"
cargo +stable fmt --all -- --check

echo "========= rustdoc"
cargo doc --no-deps --workspace --document-private-items --all-features

echo "========= clippy nightly"
cargo +nightly clippy --workspace --all-targets --all-features -- -D warnings

echo "========= tests"
# You can also adjust profile settings by using environment variables documented
# [here](https://doc.rust-lang.org/cargo/reference/environment-variables.html).
# For example, LTO for tests can be disabled with `CARGO_PROFILE_TEST_LTO=false`
# (results in less optimizations: faster building, but slower execution).
cargo +stable test --workspace --features test-debug-mode-checks
# If you also want to run large tests, include `--features include-large-tests`.

echo "========="
echo "Poor's man version of CI -- ok!"

# If you want this as a git hook:
# $ cat ../../.git/hooks/pre-commit
# #!/bin/sh
#
# cmemu_root=$(pwd)/aj370953/cmemu-framework/
#
# exec $cmemu_root/poors-man-ci.sh

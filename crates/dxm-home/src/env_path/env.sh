#!/bin/sh
# This script adds dxm to the environment PATH.
# Adapted from Rustup:
# https://github.com/rust-lang/rustup/blob/0851758bb2e7134d48b54a52f658aaccadb59de1/src/cli/self_update/env.sh
case ":${PATH}:" in
    *:"{dxm_bin}":*)
        ;;
    *)
        export PATH="{dxm_bin}:$PATH"
        ;;
esac

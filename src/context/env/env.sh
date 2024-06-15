#!/bin/sh
# This script adds DXM to the environment PATH.
# Adapted from Rustup:
# https://github.com/rust-lang/rustup/blob/6d5f0f698be1078636c2efe7a7e70e193bad5bff/src/cli/self_update/env.sh
case ":${PATH}:" in
    *:"{dxm_bin}":*)
        ;;
    *)
        export PATH="{dxm_bin}:$PATH"
        ;;
esac

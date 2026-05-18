#!/usr/bin/env bash

set -euo pipefail

info() {
    echo "info: $*"
}

error() {
    echo "error: $*" >&2
    exit 1
}

ensure_dependencies() {
    for dep in "$@"; do
        if ! command -v "$dep" >/dev/null 2>&1; then
            error "'$dep' is not installed or available"
        fi
    done
}

ensure_dependencies mktemp uname curl

on_exit() {
    if [[ -n "${download_dir:-}" ]]; then
        rm -rf "$download_dir"
    fi
}

trap on_exit EXIT
download_dir="$(mktemp -d)"

program="dxm"
repository="D4isDAVID/dxm"

os="$(uname -s)"
case "$os" in
    MINGW*|Win*)
        os="windows"

        unarchive_command="unzip"
        unarchive_options=(unzip -q -d "$download_dir")
        archive_extension="zip"
        ;;
    Linux)
        os="linux"

        unarchive_command="tar"
        unarchive_options=(-C "$download_dir" -xzf)
        archive_extension="tar.gz"
        ;;
    *)
        error "unsupported operating system '$os'"
        ;;
esac

ensure_dependencies "${unarchive_command}"

arch="$(uname -m)"
case "$arch" in
    x86_64|x86-64)
        arch="x64"
        ;;
    *)
        error "unsupported architecture '$arch'"
        ;;
esac

release="latest"
program_args=()

arg_value() {
    if [[ -z "$value" ]]; then
        error "no value provided for argument '$key'"
    fi

    echo "$value"
}

while [[ $# -gt 0 ]]; do
    key="$1"
    value="${2:-}"

    case "$key" in
        -r|--release)
            release="$(arg_value)"
            shift 2
            ;;
        -d|--install-dir)
            export DXM_HOME="$(arg_value)"
            shift 2
            ;;
        -e|--no-env-path)
            program_args+=(--no-env-path)
            shift
            ;;
        *)
            error "unrecognized argument '$key'"
            ;;
    esac
done

curl_options=(--proto '=https' --tlsv1.2 -sSf -H "X-GitHub-Api-Version: 2022-11-28")
if [[ -n "${GITHUB_PAT:-}" ]]; then
    info "using provided GITHUB_PAT for authentication"
    curl_options+=(-H "Authorization: token $GITHUB_PAT")
fi

run_curl() {
    curl "${curl_options[@]}" "$@"
}

if [[ "$release" == "latest" ]]; then
    info "fetching latest release"
    api_url="https://api.github.com/repos/$repository/releases/latest"
else
    info "fetching release $release"
    api_url="https://api.github.com/repos/$repository/releases/tags/$release"
fi

release_json="$(run_curl "$api_url")" || {
    error "failed to fetch release data, please check your network connection"
}

download_url=""
while IFS= read -r current_line; do
    if [[ "$current_line" == *'"browser_download_url":'* && "$current_line" == *"-$os-$arch."* ]]; then
        download_url="${current_line#*\"browser_download_url\": \"}"
        download_url="${download_url%\"*}"
    fi
done <<< "$release_json"

if [[ -z "$download_url" ]]; then
    error "failed to find download url for $"
fi

info "downloading $download_url"

archive="$download_dir/dxm.$archive_extension"
run_curl -L -o "$archive" "$download_url" || {
    error "failed to download $download_url"
}

binary_name="$program"
if [ "$os" = "windows" ]; then
    binary_name="${binary_name}.exe"
fi

info "extracting $binary_name"

"$unarchive_command" "${unarchive_options[@]}" "$archive" "$binary_name" || {
    error "failed to unarchive $binary_name"
}

binary_file="$download_dir/$binary_name"

info "running $program"

if [ "$os" != "windows" ]; then
    chmod +x "$binary_file"
fi

"$binary_file" self setup "${program_args[@]}"

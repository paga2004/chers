#!/bin/sh

show() {
    grep -E '^cpu MHz' /proc/cpuinfo
    cpupower frequency-info | grep "current policy"
}

if [ "$1" = "set" ]; then
    cpupower frequency-set -d ${2:-2GHz} > /dev/null || exit 2
    cpupower frequency-set -u ${2:-2GHz} > /dev/null || exit 2
elif [ "$1" = "auto" ]; then
    cpupower frequency-set -d 400MHz > /dev/null || exit 2
    cpupower frequency-set -u 4GHz > /dev/null || exit 2
elif [ "$1" = "show" ]; then
    show
else
    echo "Expected one of (set|auto|show)"
    exit 1
fi


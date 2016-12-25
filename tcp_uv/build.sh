#!/usr/bin/env bash

set -e

PROJECT_PATH=`pwd`

# downloading LibUV latest release
if [ ! -d "libuv" ]; then
    echo "Getting LibUV v1.10.1 Release files !"
    curl -O -J -L "https://github.com/libuv/libuv/archive/v1.10.1.tar.gz" > /dev/null 2>&1
    tar xzf libuv-1.10.1.tar.gz
    mv libuv-1.10.1 libuv
    cd libuv
    echo "Building LibUV"
    ./autogen.sh > /dev/null
    ./configure --prefix=`pwd`/build_tg  > /dev/null
    make -j4 > /dev/null
    make install > /dev/null
    cd "$PROJECT_PATH"
fi

echo "Building LibUV Tcp Echo Server !"
mkdir -p build
cd build
# cleaning files just in case we have something there
rm -rf *
cmake -DCMAKE_BUILD_TYPE=MinSizeRel ../
make

echo "Build Done ! Run with ./build/tcp_uv 8888"
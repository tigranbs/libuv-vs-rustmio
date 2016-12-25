# About
[Libuv](https://github.com/libuv/libuv) is a super popular cross platform _Networking Event Loop Engine writtent in C_ , it's a main Networking engine for Node.js.
[MIO](https://github.com/carllerche/mio) is a cross platform _Networking Event Loop Engine written in Rust_ with 0 cost of abstraction principle.

So because of the "war" between Rust and C/C++ in terms of performance and stability, and because of I have an application which is currently using Libuv, wanted to benchmark MIO and Libuv as a different language implementations of almost the same OS API's.
Both benchmark projects built for super easy usage, so you can run benchmarks on your machine.
It's tested for MAC and Linux.

# Usage
For making some benchmarking we need 3rd tool for just sending and reading network traffic, it's built with Go.
So the basic usage is following
#### Running MIO Tcp Echo server - you will need Rust and Cargo installed
```bash
# this will start TCP echo server on port 8888
cd tcp_mio/ && cargo run

```
#### Running Libuv Tcp Echo server
```bash
# building C code
cd tcp_uv && ./build.sh

# Starting Libuv TCP echo server on port 8888
./tcp_uv/build/tcp_uv 8888
```
#### Starting Benchmarking tool
```bash
# this will open 100 concurrent connections to 8888 port on localhost
# and will start sending "LICENSE" file content
# you can choose any file for benchmarking
go run bc.go localhost:8888 100 LICENSE
```
**NOTE:** Make use that file is not large, because you may crash your OS. During TCP Write process bytes would stay in memory until write is done.

# Results
I'll add some results for multiple OS's and instances

# Contribution
If you have ideas about what kind of applications we can write in order to make a real world benchmarking just open an issue for discussion or send ma Pull Request.

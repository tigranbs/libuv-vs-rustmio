# libuv-vs-rustmio
Some Benchmark and testing around Libuv and Rust MIO libraries

Prior to running the benchmark: `cd tcp_mio/ && cargo run`.

Example benchmark: `go run bc.go localhost:8888 100 LICENSE`. This will run the benchmark with 100 connections using the contents of the LICENSE file as the data to echo.

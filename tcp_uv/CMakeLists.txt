cmake_minimum_required(VERSION 3.6)
project(tcp_uv)

set(CMAKE_CXX_STANDARD 11)
set(LIBUV_BUILD_PATH ${CMAKE_SOURCE_DIR}/libuv/build_tg)

include_directories(${LIBUV_BUILD_PATH}/include)

set(SOURCE_FILES main.cpp)
add_executable(tcp_uv ${SOURCE_FILES})
target_link_libraries(tcp_uv
        ${LIBUV_BUILD_PATH}/lib/libuv.a
        )
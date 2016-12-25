#include <cstdlib>
#include "uv.h"

void on_connection(uv_stream_t* server, int status);
void on_close_connection(uv_handle_t* handle);
void on_alloc_callback(uv_handle_t* handle, size_t suggested_size, uv_buf_t* buf);
void on_read_callback(uv_stream_t* stream, ssize_t nread, const uv_buf_t* buf);
void on_write_callback(uv_write_t* req, int status);

#define READABLE_BUFFER_SIZE 65000

// allocated buffer space for faster read process
char readable_buffer[READABLE_BUFFER_SIZE];

int main(int argc, char *argv[]) {
    // making LibUV loop
    uv_loop_t *loop;
    // making tcp server handle
    uv_tcp_t tcp_server;

    loop = (uv_loop_t*) malloc(sizeof(uv_loop_t));
    // init loop
    uv_loop_init(loop);

    // resolving address to bind TCP server
    struct sockaddr_in addr;
    int r = uv_ip4_addr("0.0.0.0", atoi(argv[1]), &addr);
    if(r) {
        fprintf(stderr, "Unable to Resolve given address on port %s -> %s", argv[1], uv_strerror(r));
        exit(1);
    }

    // init TCP server for our loop
    uv_tcp_init(loop, &tcp_server);

    // binding TCP server
    r = uv_tcp_bind(&tcp_server, (const struct sockaddr*)&addr, 0);
    if(r) {
        fprintf(stderr, "Unable to bind TCP server on port %s -> %s", argv[1], uv_strerror(r));
        exit(1);
    }

    // listening TCP server on given port
    r = uv_listen((uv_stream_t*) &tcp_server, 1000, on_connection);
    if(r) {
        fprintf(stderr, "Unable to Listen TCP server on port %s -> %s", argv[1], uv_strerror(r));
        exit(1);
    }

    // starting loop
    uv_run(loop, UV_RUN_DEFAULT);

    return 0;
}

void on_connection(uv_stream_t* server, int status) {
    if(status) {
        fprintf(stderr, "Unable to accept TCP connection -> %s", uv_strerror(status));
        return;
    }

    // making client connection handle
    uv_tcp_t *client = (uv_tcp_t*)malloc(sizeof(uv_tcp_t));
    uv_tcp_init(server->loop, client);

    if(uv_accept(server, (uv_stream_t*) client) == 0) {
        status = uv_read_start((uv_stream_t*)client, on_alloc_callback, on_read_callback);
        if(status) {
            uv_close((uv_handle_t*) client, on_close_connection);
            fprintf(stderr, "Unable to start reading from TCP connection -> %s", uv_strerror(status));
            return;
        }
    } else {
        // closing connection handle if we can't accept it
        uv_close((uv_handle_t*) client, on_close_connection);
    }
}

void on_close_connection(uv_handle_t* handle) {
    // if we got here then connection is closed
    // so we need just to FREE handle from memory
    free(handle);
}

void on_alloc_callback(uv_handle_t* handle, size_t suggested_size, uv_buf_t* buf) {
    buf->base = readable_buffer;
    buf->len = READABLE_BUFFER_SIZE;
}

void on_read_callback(uv_stream_t* stream, ssize_t nread, const uv_buf_t* buf) {
    if(nread < 0) {
        if(nread != UV_EOF) {
            fprintf(stderr, "Connection Reading Error -> %s", uv_strerror(nread));
        }

        // closing connection handle if we can't accept it
        uv_close((uv_handle_t*) stream, on_close_connection);
        return;
    }

    // if we got here then we have some data in buffer
    // we are doing basic TCP Echo server
    // so we need just to copy buffer that we got and write to TCP connection
    uv_write_t *write_handle = (uv_write_t*) malloc(sizeof(uv_write_t));
    uv_buf_t *writable_buf = (uv_buf_t*) malloc(sizeof(uv_buf_t));
    writable_buf->base = (char*) malloc((size_t)nread);
    writable_buf->len = (size_t)nread;
    memcpy(writable_buf->base, buf->base, writable_buf->len);
    write_handle->data = writable_buf;
    uv_write(write_handle, stream, writable_buf, 1, on_write_callback);
}

void on_write_callback(uv_write_t* req, int status) {
    // if we got here then our write completed or we got an error
    // in any case we need to delete allocated data from memory
    uv_buf_t *buf = (uv_buf_t*) req->data;
    free(buf->base);
    free(buf);
    free(req);
}
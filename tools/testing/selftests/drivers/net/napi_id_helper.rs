//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/drivers/net/napi_id_helper.c
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: GPL-2.0

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    struct sockaddr_storage address;
    struct addrinfo *result;
    struct addrinfo hints;
    unsigned int napi_id;
    socklen_t addr_len;
    socklen_t optlen;
    char buf[1024];
    let mut opt: c_int = 1;
    int family;
    int server;
    int client;
    int ret;
    memset(&hints, 0, sizeof(hints));
    hints.ai_family = AF_UNSPEC;
    hints.ai_socktype = SOCK_STREAM;
    hints.ai_flags = AI_PASSIVE;
    ret = getaddrinfo(argv[1], argv[2], &hints, &result);
    if (ret != 0) {
    fprintf(stderr, "getaddrinfo: %s\n", gai_strerror(ret));
    return 1;
    }
    family = result.ai_family;
    addr_len = result.ai_addrlen;
    server = socket(family, SOCK_STREAM, IPPROTO_TCP);
    if (server < 0) {
    perror("socket creation failed");
    freeaddrinfo(result);
    if (errno == EAFNOSUPPORT)
    return -1;
    return 1;
    }
    if (setsockopt(server, SOL_SOCKET, SO_REUSEADDR, &opt, sizeof(opt))) {
    perror("setsockopt");
    freeaddrinfo(result);
    return 1;
    }
    memcpy(&address, result.ai_addr, result.ai_addrlen);
    freeaddrinfo(result);
    if (bind(server, (struct sockaddr *)&address, addr_len) < 0) {
    perror("bind failed");
    return 1;
    }
    if (listen(server, 1) < 0) {
    perror("listen");
    return 1;
    }
    ksft_ready();
    client = accept(server, core::ptr::null_mut(), 0);
    if (client < 0) {
    perror("accept");
    return 1;
    }
    optlen = sizeof(napi_id);
    ret = getsockopt(client, SOL_SOCKET, SO_INCOMING_NAPI_ID, &napi_id,
    &optlen);
    if (ret != 0) {
    perror("getsockopt");
    return 1;
    }
    read(client, buf, 1024);
    ksft_wait();
    if (napi_id == 0) {
    fprintf(stderr, "napi ID is 0\n");
    return 1;
    }
    close(client);
    close(server);
    return 0;
    }

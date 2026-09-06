//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/net/sk_bind_sendto_listen.c
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
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    int fd1, fd2, one = 1;
    struct sockaddr_in6 bind_addr = {
    .sin6_family = AF_INET6,
    .sin6_port = htons(20000),
    .sin6_flowinfo = htonl(0),
    .sin6_addr = {},
    .sin6_scope_id = 0,
    };
    inet_pton(AF_INET6, "::", &bind_addr.sin6_addr);
    fd1 = socket(AF_INET6, SOCK_STREAM, IPPROTO_IP);
    if (fd1 < 0) {
    error(1, errno, "socket fd1");
    return -1;
    }
    if (setsockopt(fd1, SOL_SOCKET, SO_REUSEADDR, &one, sizeof(one))) {
    error(1, errno, "setsockopt(SO_REUSEADDR) fd1");
    goto out_err1;
    }
    if (bind(fd1, (struct sockaddr *)&bind_addr, sizeof(bind_addr))) {
    error(1, errno, "bind fd1");
    goto out_err1;
    }
    if (sendto(fd1, core::ptr::null_mut(), 0, MSG_FASTOPEN, (struct sockaddr *)&bind_addr,
    sizeof(bind_addr))) {
    error(1, errno, "sendto fd1");
    goto out_err1;
    }
    fd2 = socket(AF_INET6, SOCK_STREAM, IPPROTO_IP);
    if (fd2 < 0) {
    error(1, errno, "socket fd2");
    goto out_err1;
    }
    if (setsockopt(fd2, SOL_SOCKET, SO_REUSEADDR, &one, sizeof(one))) {
    error(1, errno, "setsockopt(SO_REUSEADDR) fd2");
    goto out_err2;
    }
    if (bind(fd2, (struct sockaddr *)&bind_addr, sizeof(bind_addr))) {
    error(1, errno, "bind fd2");
    goto out_err2;
    }
    if (sendto(fd2, core::ptr::null_mut(), 0, MSG_FASTOPEN, (struct sockaddr *)&bind_addr,
    sizeof(bind_addr)) != -1) {
    error(1, errno, "sendto fd2");
    goto out_err2;
    }
    if (listen(fd2, 0)) {
    error(1, errno, "listen");
    goto out_err2;
    }
    close(fd2);
    close(fd1);
    return 0;
    out_err2:
    close(fd2);
    out_err1:
    close(fd1);
    return -1;
    }
